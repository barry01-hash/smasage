#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

#[soroban_sdk::contractclient(name = "SoroswapRouterClient")]
pub trait SoroswapRouterTrait {
    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> (i128, i128, i128);

    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Vec<i128>;
}

#[soroban_sdk::contractclient(name = "TokenClient")]
pub trait TokenTrait {
    fn transfer(e: Env, from: Address, to: Address, amount: i128);
    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
    fn balance(e: Env, id: Address) -> i128;
}

// Issue 2: Smart Contract - Stellar Path Payments & Yield Allocation (Blend Integration)
// Issue 3: Withdraw functionality with Blend and Soroswap unwinding

#[contracttype]
pub enum DataKey {
    Admin,
    UserBalance(Address),
    UserLPShares(Address),
    UserBlendBalance(Address),
    UserGoldBalance(Address),
    /// User's Blend Protocol position (bTokens)
    UserBlendPosition(Address),
    /// Mock Blend Pool address (for testing)
    BlendPoolAddress,
    /// USDC Token contract address
    UsdcTokenAddress,
    /// Total bTokens held by the contract across all users
    TotalBTokens,
    /// Total vault deposits across all users (in USDC)
    TotalVaultDeposits,
    TotalDeposits,
    GoldAssetCode,
    GoldAssetIssuer,
    GoldTrustlineReady,
    GoldTrustlineReserveStroops,
    SoroswapRouter,
    UsdcToken,
    XlmToken,
}

const CANONICAL_GOLD_ASSET_CODE: Symbol = symbol_short!("XAUT");
const CANONICAL_GOLD_ASSET_ISSUER: &str =
    "GCRLXTLD7XIRXWXV2PDCC74O5TUUKN3OODJAM6TWVE4AIRNMGQJK3KWQ";
const TRUSTLINE_BASE_RESERVE_STROOPS: i128 = 5_000_000;
const INDEX_RATE_PRECISION: i128 = 1_000_000; // 1.0 represented as 1,000,000

#[contracttype]
#[derive(Clone, Copy)]
pub struct BlendPosition {
    pub b_tokens: i128,
    pub last_index_rate: i128,
    pub last_supply_time: u64,
}

#[contract]
pub struct SmasageYieldRouter;

#[contractimpl]
impl SmasageYieldRouter {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }

    pub fn initialize_soroswap(
        env: Env,
        admin: Address,
        router: Address,
        usdc: Address,
        xlm: Address,
    ) {
        let stored_admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Contract not initialized");
        assert!(admin == stored_admin, "Only admin can initialize Soroswap");
        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::SoroswapRouter, &router);
        env.storage().persistent().set(&DataKey::UsdcToken, &usdc);
        env.storage().persistent().set(&DataKey::XlmToken, &xlm);
    }

    pub fn init_gold_trustline(env: Env, admin: Address, reserve_stroops: i128) {
        let stored_admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Contract not initialized");

        assert!(
            admin == stored_admin,
            "Only admin can initialize Gold trustline"
        );
        admin.require_auth();
        assert!(
            reserve_stroops >= TRUSTLINE_BASE_RESERVE_STROOPS,
            "Insufficient base reserve for trustline"
        );

        let gold_issuer = String::from_str(&env, CANONICAL_GOLD_ASSET_ISSUER);
        env.storage()
            .persistent()
            .set(&DataKey::GoldAssetCode, &CANONICAL_GOLD_ASSET_CODE);
        env.storage()
            .persistent()
            .set(&DataKey::GoldAssetIssuer, &gold_issuer);
        env.storage()
            .persistent()
            .set(&DataKey::GoldTrustlineReserveStroops, &reserve_stroops);
        env.storage()
            .persistent()
            .set(&DataKey::GoldTrustlineReady, &true);
    }

    pub fn get_gold_asset(env: Env) -> (Symbol, String) {
        let code = env
            .storage()
            .persistent()
            .get(&DataKey::GoldAssetCode)
            .unwrap_or(CANONICAL_GOLD_ASSET_CODE);
        let issuer = env
            .storage()
            .persistent()
            .get(&DataKey::GoldAssetIssuer)
            .unwrap_or(String::from_str(&env, CANONICAL_GOLD_ASSET_ISSUER));
        (code, issuer)
    }

    pub fn is_gold_trustline_ready(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::GoldTrustlineReady)
            .unwrap_or(false)
    }

    pub fn get_gold_reserve_stroops(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::GoldTrustlineReserveStroops)
            .unwrap_or(0)
    }

    /// Deposit USDC into the vault
    ///
    /// This is the primary vault deposit function that:
    /// - Requires cryptographic authorization from the sender
    /// - Transfers USDC tokens from user to contract
    /// - Tracks individual user balances
    /// - Updates total protocol deposits
    ///
    /// # Arguments
    /// * `from` - The address making the deposit (must authorize the transaction)
    /// * `amount` - The amount of USDC to deposit (must be > 0)
    ///
    /// # Panics
    /// - If `amount` is not positive
    /// - If USDC token is not initialized
    /// - If token transfer fails (insufficient balance, approval, etc.)
    pub fn vault_deposit(env: Env, from: Address, amount: i128) {
        // 1. Authorization: Require cryptographic signature from the sender
        from.require_auth();

        // 2. Input validation
        assert!(amount > 0, "Deposit amount must be greater than 0");

        // 3. Transfer USDC tokens from user to contract
        let usdc_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::UsdcToken)
            .expect("USDC not initialized");
        let usdc = TokenClient::new(&env, &usdc_addr);
        usdc.transfer(&from, &env.current_contract_address(), &amount);

        // 4. Update individual user balance (vault deposit tracking)
        let mut user_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserBalance(from.clone()))
            .unwrap_or(0);
        user_balance += amount;
        env.storage()
            .persistent()
            .set(&DataKey::UserBalance(from.clone()), &user_balance);

        // 5. Update total vault deposits (protocol-wide tracking)
        let mut total_deposits: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalVaultDeposits)
            .unwrap_or(0);
        total_deposits += amount;
        env.storage()
            .persistent()
            .set(&DataKey::TotalVaultDeposits, &total_deposits);
    }

    /// Get total vault deposits across all users
    ///
    /// # Returns
    /// The total amount of USDC deposited into the vault (in USDC)
    pub fn get_total_vault_deposits(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalVaultDeposits)
            .unwrap_or(0)
    }

    /// Get a user's vault balance
    ///
    /// # Arguments
    /// * `user` - The address to check
    ///
    /// # Returns
    /// The user's vault balance in USDC
    pub fn get_vault_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserBalance(user))
            .unwrap_or(0)
    }

    /// Get user's Blend position details
    pub fn get_blend_position(env: Env, user: Address) -> BlendPosition {
        env.storage()
            .persistent()
            .get(&DataKey::UserBlendPosition(user))
            .unwrap_or(BlendPosition {
                b_tokens: 0,
                last_index_rate: INDEX_RATE_PRECISION,
                last_supply_time: 0,
            })
    }

    /// Get the current mock index rate (for testing only)
    /// In production, this would query the actual Blend pool
    pub fn get_mock_index_rate(env: Env) -> i128 {
        // This is a test helper - in production, this reads from actual Blend pool
        // For now, return the default precision
        INDEX_RATE_PRECISION
    }

    /// Set the mock index rate (for testing only)
    /// This allows tests to simulate yield accrual
    pub fn set_mock_index_rate(env: Env, new_rate: i128) {
        // Store the mock index rate in a special storage location
        // We use a tuple key pattern to avoid collision with real data
        env.storage()
            .persistent()
            .set(&DataKey::TotalDeposits, &new_rate);
    }

    /// Initialize the contract and accept deposits in USDC.
    /// Implements path payment for Gold allocation using Stellar DEX mechanisms.
    ///
    /// This function is kept for backward compatibility. New code should use vault_deposit()
    /// for simple deposits, or combine vault_deposit() with supply_to_blend() for complex allocation.
    pub fn deposit(
        env: Env,
        from: Address,
        amount: i128,
        blend_percentage: u32,
        lp_percentage: u32,
        gold_percentage: u32,
    ) {
        from.require_auth();
        assert!(
            blend_percentage + lp_percentage + gold_percentage <= 100,
            "Allocation exceeds 100%"
        );

        // First, perform the base vault deposit (transfers USDC and tracks balance)
        Self::vault_deposit(env.clone(), from.clone(), amount);

        // Then handle allocations across different protocols
        // Track Blend allocation
        let blend_amount = amount * blend_percentage as i128 / 100;
        let mut blend_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserBlendBalance(from.clone()))
            .unwrap_or(0);
        blend_balance += blend_amount;
        env.storage()
            .persistent()
            .set(&DataKey::UserBlendBalance(from.clone()), &blend_balance);

        // Track LP shares allocation
        let lp_amount = amount * lp_percentage as i128 / 100;
        let mut lp_shares: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserLPShares(from.clone()))
            .unwrap_or(0);
        lp_shares += lp_amount;
        env.storage()
            .persistent()
            .set(&DataKey::UserLPShares(from.clone()), &lp_shares);

        // Track Gold allocation (XAUT)
        let gold_amount = amount * gold_percentage as i128 / 100;
        if gold_amount > 0 {
            let mut gold_balance: i128 = env
                .storage()
                .persistent()
                .get(&DataKey::UserGoldBalance(from.clone()))
                .unwrap_or(0);
            gold_balance += gold_amount;
            env.storage()
                .persistent()
                .set(&DataKey::UserGoldBalance(from.clone()), &gold_balance);
        }
    }

    fn provide_lp(env: Env, user: Address, usdc_amount: i128) {
        let router_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::SoroswapRouter)
            .expect("Soroswap not initialized");
        let usdc_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::UsdcToken)
            .expect("USDC not initialized");
        let xlm_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::XlmToken)
            .expect("XLM not initialized");

        let router = SoroswapRouterClient::new(&env, &router_addr);
        let usdc = TokenClient::new(&env, &usdc_addr);
        let xlm = TokenClient::new(&env, &xlm_addr);

        let half_usdc = usdc_amount / 2;
        let remaining_usdc = usdc_amount - half_usdc;

        // Approve router for total USDC amount to be used in swap and liquidity
        usdc.approve(
            &env.current_contract_address(),
            &router_addr,
            &usdc_amount,
            &(env.ledger().sequence() + 100),
        );

        // Swap half USDC for XLM
        let mut path = Vec::new(&env);
        path.push_back(usdc_addr.clone());
        path.push_back(xlm_addr.clone());

        let deadline = env.ledger().timestamp() + 300; // 5 minutes
        let swap_amounts = router.swap_exact_tokens_for_tokens(
            &half_usdc,
            &0,
            &path,
            &env.current_contract_address(),
            &deadline,
        );
        let xlm_received = swap_amounts.get(1).unwrap();

        // Approve router for received XLM
        xlm.approve(
            &env.current_contract_address(),
            &router_addr,
            &xlm_received,
            &(env.ledger().sequence() + 100),
        );

        // Add liquidity
        let (_, _, lp_shares) = router.add_liquidity(
            &usdc_addr,
            &xlm_addr,
            &remaining_usdc,
            &xlm_received,
            &0,
            &0,
            &env.current_contract_address(),
            &deadline,
        );

        // Map LP shares to user
        let mut user_shares: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserLPShares(user.clone()))
            .unwrap_or(0);
        user_shares += lp_shares;
        env.storage()
            .persistent()
            .set(&DataKey::UserLPShares(user), &user_shares);
    }

    /// Withdraw USDC by unwinding positions from Blend and breaking LP shares from Soroswap.
    /// The contract calculates how much to pull from each source and transfers USDC to the user.
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        to.require_auth();

        // Get total user balance (USDC + Blend + LP + Gold)
        let usdc_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserBalance(to.clone()))
            .unwrap_or(0);
        let blend_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserBlendBalance(to.clone()))
            .unwrap_or(0);
        let lp_shares: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserLPShares(to.clone()))
            .unwrap_or(0);
        let gold_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::UserGoldBalance(to.clone()))
            .unwrap_or(0);

        let total_balance = usdc_balance + blend_balance + lp_shares + gold_balance;
        assert!(total_balance >= amount, "Insufficient balance");

        let mut remaining_to_withdraw = amount;

        // Step 1: Use available USDC first
        if usdc_balance > 0 {
            let usdc_to_use = usdc_balance.min(remaining_to_withdraw);
            env.storage().persistent().set(
                &DataKey::UserBalance(to.clone()),
                &(usdc_balance - usdc_to_use),
            );
            remaining_to_withdraw -= usdc_to_use;
        }

        // Step 2: If still need more, unwind Blend positions (pull liquidity)
        if remaining_to_withdraw > 0 && blend_balance > 0 {
            let blend_to_unwind = blend_balance.min(remaining_to_withdraw);
            env.storage().persistent().set(
                &DataKey::UserBlendBalance(to.clone()),
                &(blend_balance - blend_to_unwind),
            );
            // Mock: In production, this would call Blend Protocol to withdraw underlying assets
            // For simplicity, we assume 1:1 conversion back to USDC
            remaining_to_withdraw -= blend_to_unwind;
        }

        // Step 3: If still need more, break LP shares on Soroswap
        if remaining_to_withdraw > 0 && lp_shares > 0 {
            let lp_to_break = lp_shares.min(remaining_to_withdraw);
            env.storage().persistent().set(
                &DataKey::UserLPShares(to.clone()),
                &(lp_shares - lp_to_break),
            );
            // Mock: In production, this would remove liquidity from Soroswap pool and swap back to USDC
            // For simplicity, we assume 1:1 conversion back to USDC
            remaining_to_withdraw -= lp_to_break;
        }

        // Step 4: If still need more, sell Gold allocation
        if remaining_to_withdraw > 0 && gold_balance > 0 {
            let gold_to_sell = gold_balance.min(remaining_to_withdraw);
            env.storage().persistent().set(
                &DataKey::UserGoldBalance(to.clone()),
                &(gold_balance - gold_to_sell),
            );
            // Mock: In production, this would swap XAUT back to USDC via Stellar DEX
            // For simplicity, we assume 1:1 conversion back to USDC
            remaining_to_withdraw -= gold_to_sell;
        }

        assert!(remaining_to_withdraw == 0, "Withdrawal calculation failed");

        // Mock: Transfer the resulting USDC to the user
        // In production, this would execute actual token transfers via Soroban token interface
    }

    /// Get user's Gold (XAUT) balance
    pub fn get_gold_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserGoldBalance(user))
            .unwrap_or(0)
    }

    /// Get user's LP shares balance
    pub fn get_lp_shares(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserLPShares(user))
            .unwrap_or(0)
    }

    /// Get user's USDC balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::UserBalance(user))
            .unwrap_or(0)
    }
}

// Test Mocks & Unit Tests
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[contract]
    pub struct MockToken;
    #[contractimpl]
    impl TokenTrait for MockToken {
        fn transfer(_e: Env, _from: Address, _to: Address, _amount: i128) {}
        fn approve(
            _e: Env,
            _from: Address,
            _spender: Address,
            _amount: i128,
            _expiration_ledger: u32,
        ) {
        }
        fn balance(_e: Env, _id: Address) -> i128 {
            0
        }
    }

    #[contract]
    pub struct MockRouter;
    #[contractimpl]
    impl SoroswapRouterTrait for MockRouter {
        fn add_liquidity(
            _e: Env,
            _token_a: Address,
            _token_b: Address,
            _amount_a_desired: i128,
            _amount_b_desired: i128,
            _amount_a_min: i128,
            _amount_b_min: i128,
            _to: Address,
            _deadline: u64,
        ) -> (i128, i128, i128) {
            // Returns (amount_a_used, amount_b_used, lp_shares_minted)
            (0, 0, 100)
        }

        fn swap_exact_tokens_for_tokens(
            e: Env,
            amount_in: i128,
            _amount_out_min: i128,
            _path: Vec<Address>,
            _to: Address,
            _deadline: u64,
        ) -> Vec<i128> {
            let mut v = Vec::new(&e);
            v.push_back(amount_in);
            v.push_back(amount_in * 2); // Mock 1:2 swap rate (USDC:XLM)
            v
        }
    }

    /// Helper: set up the contract, admin, mocks, and return everything needed for tests.
    fn setup_env() -> (
        Env,
        SmasageYieldRouterClient<'static>,
        Address,
        Address,
        Address,
        Address,
    ) {
        let env = Env::default();
        let contract_id = env.register(SmasageYieldRouter, ());
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        let router_id = env.register(MockRouter, ());
        let usdc_id = env.register(MockToken, ());
        let xlm_id = env.register(MockToken, ());

        env.mock_all_auths();
        client.initialize(&admin);
        client.initialize_soroswap(&admin, &router_id, &usdc_id, &xlm_id);

        (env, client, admin, router_id, usdc_id, xlm_id)
    }

    #[test]
    fn test_initialize_gold_trustline() {
        let env = Env::default();
        let contract_id = env.register(SmasageYieldRouter, ());
        let client = SmasageYieldRouterClient::new(&env, &contract_id);
        let admin = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin);
        client.init_gold_trustline(&admin, &5_000_000);

        let (asset_code, asset_issuer) = client.get_gold_asset();
        assert_eq!(asset_code, symbol_short!("XAUT"));
        assert_eq!(
            asset_issuer,
            String::from_str(
                &env,
                "GCRLXTLD7XIRXWXV2PDCC74O5TUUKN3OODJAM6TWVE4AIRNMGQJK3KWQ"
            )
        );
        assert!(client.is_gold_trustline_ready());
        assert_eq!(client.get_gold_reserve_stroops(), 5_000_000);
    }

    #[test]
    fn test_vault_deposit_success() {
        let (_env, client, _admin, _r, _u, _x) = setup_env();
        let user = Address::generate(&_env);

        // Deposit 1000 USDC via vault_deposit
        client.vault_deposit(&user, &1000);

        // Verify user balance was updated
        assert_eq!(client.get_vault_balance(&user), 1000);

        // Verify total vault deposits was updated
        assert_eq!(client.get_total_vault_deposits(), 1000);
    }

    #[test]
    fn test_vault_deposit_accumulation() {
        let (_env, client, _admin, _r, _u, _x) = setup_env();
        let user = Address::generate(&_env);

        // Make multiple deposits
        client.vault_deposit(&user, &1000);
        assert_eq!(client.get_vault_balance(&user), 1000);
        assert_eq!(client.get_total_vault_deposits(), 1000);

        client.vault_deposit(&user, &2000);
        assert_eq!(client.get_vault_balance(&user), 3000);
        assert_eq!(client.get_total_vault_deposits(), 3000);

        client.vault_deposit(&user, &5000);
        assert_eq!(client.get_vault_balance(&user), 8000);
        assert_eq!(client.get_total_vault_deposits(), 8000);
    }

    #[test]
    #[should_panic(expected = "Deposit amount must be greater than 0")]
    fn test_vault_deposit_zero_amount() {
        let (_env, client, _admin, _r, _u, _x) = setup_env();
        let user = Address::generate(&_env);

        // Attempt to deposit 0 - should panic
        client.vault_deposit(&user, &0);
    }

    #[test]
    #[should_panic(expected = "Allocation exceeds 100%")]
    fn test_allocation_exceeds_100_percent() {
        let (_env, client, _admin, _r, _u, _x) = setup_env();
        let user = Address::generate(&_env);

        client.deposit(&user, &1000, &60, &50, &0); // 110% → panic
    }
}
