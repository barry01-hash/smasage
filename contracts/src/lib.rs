#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, IntoVal,
};

// Issue 2: Smart Contract - Stellar Path Payments & Yield Allocation (Blend Integration)
// Issue 3: Withdraw functionality with Blend and Soroswap unwinding

#[contracttype]
pub enum DataKey {
    Admin,
    UserBalance(Address),
    UserLPShares(Address),
    UserBlendBalance(Address),
    UserGoldBalance(Address),
    TotalDeposits,
    GoldAssetCode,
    GoldAssetIssuer,
    GoldTrustlineReady,
    GoldTrustlineReserveStroops,
    SoroswapRouter,
    UsdcAsset,
    PairedAsset,
}

const CANONICAL_GOLD_ASSET_CODE: Symbol = symbol_short!("XAUT");
const CANONICAL_GOLD_ASSET_ISSUER: &str = "GCRLXTLD7XIRXWXV2PDCC74O5TUUKN3OODJAM6TWVE4AIRNMGQJK3KWQ";
const TRUSTLINE_BASE_RESERVE_STROOPS: i128 = 5_000_000;

#[contract]
pub struct SmasageYieldRouter;

#[contractimpl]
impl SmasageYieldRouter {
    pub fn initialize(env: Env, admin: Address, usdc: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::UsdcAsset, &usdc);
    }

    pub fn configure_soroswap(env: Env, admin: Address, router: Address, paired_asset: Address) {
        let stored_admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Contract not initialized");
        assert!(admin == stored_admin, "Only admin can configure Soroswap");
        admin.require_auth();

        env.storage().persistent().set(&DataKey::SoroswapRouter, &router);
        env.storage().persistent().set(&DataKey::PairedAsset, &paired_asset);
    }

    pub fn init_gold_trustline(env: Env, admin: Address, reserve_stroops: i128) {
        let stored_admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Contract not initialized");

        assert!(admin == stored_admin, "Only admin can initialize Gold trustline");
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

    /// Initialize the contract and accept deposits in USDC.
    /// Implements path payment for Gold allocation using Stellar DEX mechanisms.
    pub fn deposit(env: Env, from: Address, amount: i128, blend_percentage: u32, lp_percentage: u32, gold_percentage: u32) {
        from.require_auth();
        assert!(blend_percentage + lp_percentage + gold_percentage <= 100, "Allocation exceeds 100%");
        
        let mut balance: i128 = env.storage().persistent().get(&DataKey::UserBalance(from.clone())).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&DataKey::UserBalance(from.clone()), &balance);
        
        // Track Blend allocation
        let blend_amount = amount * blend_percentage as i128 / 100;
        let mut blend_balance: i128 = env.storage().persistent().get(&DataKey::UserBlendBalance(from.clone())).unwrap_or(0);
        blend_balance += blend_amount;
        env.storage().persistent().set(&DataKey::UserBlendBalance(from.clone()), &blend_balance);
        
        // Track LP shares allocation
        let lp_amount = amount * lp_percentage as i128 / 100;
        let mut lp_shares: i128 = env.storage().persistent().get(&DataKey::UserLPShares(from.clone())).unwrap_or(0);
        lp_shares += lp_amount;
        env.storage().persistent().set(&DataKey::UserLPShares(from.clone()), &lp_shares);
        
        // Track Gold allocation (XAUT)
        let gold_amount = amount * gold_percentage as i128 / 100;
        if gold_amount > 0 {
            // Execute path payment: USDC -> XAUT via Stellar DEX
            // In production, this would use Soroban's path payment strict receive
            // to find the best route through the Stellar DEX order books
            let mut gold_balance: i128 = env.storage().persistent().get(&DataKey::UserGoldBalance(from.clone())).unwrap_or(0);
            gold_balance += gold_amount;
            env.storage().persistent().set(&DataKey::UserGoldBalance(from.clone()), &gold_balance);
        }
        
        // Mock: Here we would route `blend_percentage` to the Blend protocol
<<<<<<< HEAD
        // Mock: Here we would route `lp_percentage` to Soroswap Pool
        // Mock: Path payment executed for `gold_percentage` to acquire XAUT
=======
        
        if lp_percentage > 0 {
            let lp_amount = (amount * lp_percentage as i128) / 100;
            let usdc_asset: Address = env.storage().persistent().get(&DataKey::UsdcAsset).expect("USDC not configured");
            let router_address: Address = env.storage().persistent().get(&DataKey::SoroswapRouter).expect("Soroswap Router not configured");
            let paired_asset: Address = env.storage().persistent().get(&DataKey::PairedAsset).expect("Paired Asset not configured");

            let half_usdc = lp_amount / 2;
            let remaining_usdc = lp_amount - half_usdc;

            // 1. Swap half USDC for Paired Asset (e.g. XLM)
            let mut path = soroban_sdk::Vec::new(&env);
            path.push_back(usdc_asset.clone());
            path.push_back(paired_asset.clone());

            let deadline = env.ledger().timestamp() + 300; // 5 minute deadline
            
            let swap_result: soroban_sdk::Vec<i128> = env.invoke_contract(
                &router_address,
                &soroban_sdk::Symbol::new(&env, "swap_exact_tokens_for_tokens"),
                soroban_sdk::vec![
                    &env,
                    half_usdc.into_val(&env),
                    0i128.into_val(&env),
                    path.into_val(&env),
                    env.current_contract_address().into_val(&env),
                    deadline.into_val(&env),
                ],
            );
            
            let paired_amount = swap_result.get(swap_result.len() - 1).unwrap();

            // 2. Add Liquidity
            let liquidity_result: (i128, i128, i128) = env.invoke_contract(
                &router_address,
                &soroban_sdk::Symbol::new(&env, "add_liquidity"),
                soroban_sdk::vec![
                    &env,
                    usdc_asset.into_val(&env),
                    paired_asset.into_val(&env),
                    remaining_usdc.into_val(&env),
                    paired_amount.into_val(&env),
                    0i128.into_val(&env),
                    0i128.into_val(&env),
                    env.current_contract_address().into_val(&env),
                    deadline.into_val(&env),
                ],
            );

            let lp_shares = liquidity_result.2;

            // 3. Track LP Shares for user
            let mut user_lp_shares: i128 = env.storage().persistent().get(&DataKey::UserLPShares(from.clone())).unwrap_or(0);
            user_lp_shares += lp_shares;
            env.storage().persistent().set(&DataKey::UserLPShares(from.clone()), &user_lp_shares);
        }
>>>>>>> 46ab13d (Implement Soroswap LP Integration (Issue 2.3) with 50/50 split logic and LP share tracking)
    }

    /// Withdraw USDC by unwinding positions from Blend and breaking LP shares from Soroswap.
    /// The contract calculates how much to pull from each source and transfers USDC to the user.
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        to.require_auth();
        
        // Get total user balance (USDC + Blend + LP + Gold)
        let usdc_balance: i128 = env.storage().persistent().get(&DataKey::UserBalance(to.clone())).unwrap_or(0);
        let blend_balance: i128 = env.storage().persistent().get(&DataKey::UserBlendBalance(to.clone())).unwrap_or(0);
        let lp_shares: i128 = env.storage().persistent().get(&DataKey::UserLPShares(to.clone())).unwrap_or(0);
        let gold_balance: i128 = env.storage().persistent().get(&DataKey::UserGoldBalance(to.clone())).unwrap_or(0);
        
        let total_balance = usdc_balance + blend_balance + lp_shares + gold_balance;
        assert!(total_balance >= amount, "Insufficient balance");
        
        let mut remaining_to_withdraw = amount;
        
        // Step 1: Use available USDC first
        if usdc_balance > 0 {
            let usdc_to_use = usdc_balance.min(remaining_to_withdraw);
            env.storage().persistent().set(&DataKey::UserBalance(to.clone()), &(usdc_balance - usdc_to_use));
            remaining_to_withdraw -= usdc_to_use;
        }
        
        // Step 2: If still need more, unwind Blend positions (pull liquidity)
        if remaining_to_withdraw > 0 && blend_balance > 0 {
            let blend_to_unwind = blend_balance.min(remaining_to_withdraw);
            env.storage().persistent().set(&DataKey::UserBlendBalance(to.clone()), &(blend_balance - blend_to_unwind));
            // Mock: In production, this would call Blend Protocol to withdraw underlying assets
            // For simplicity, we assume 1:1 conversion back to USDC
            remaining_to_withdraw -= blend_to_unwind;
        }
        
        // Step 3: If still need more, break LP shares on Soroswap
        if remaining_to_withdraw > 0 && lp_shares > 0 {
            let lp_to_break = lp_shares.min(remaining_to_withdraw);
            env.storage().persistent().set(&DataKey::UserLPShares(to.clone()), &(lp_shares - lp_to_break));
            // Mock: In production, this would remove liquidity from Soroswap pool and swap back to USDC
            // For simplicity, we assume 1:1 conversion back to USDC
            remaining_to_withdraw -= lp_to_break;
        }
        
        // Step 4: If still need more, sell Gold allocation
        if remaining_to_withdraw > 0 && gold_balance > 0 {
            let gold_to_sell = gold_balance.min(remaining_to_withdraw);
            env.storage().persistent().set(&DataKey::UserGoldBalance(to.clone()), &(gold_balance - gold_to_sell));
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
        env.storage().persistent().get(&DataKey::UserGoldBalance(user)).unwrap_or(0)
    }

    /// Get user's LP shares balance
    pub fn get_lp_shares(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&DataKey::UserLPShares(user)).unwrap_or(0)
    }

    /// Get user's USDC balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&DataKey::UserBalance(user)).unwrap_or(0)
    }

    pub fn get_lp_shares(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&DataKey::UserLPShares(user)).unwrap_or(0)
    }
}

// Basic Test Mock
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String, Val};

    // Mock contract for Soroswap Router
    #[contract]
    pub struct MockSoroswapRouter;

    #[contractimpl]
    impl MockSoroswapRouter {
        pub fn swap_exact_tokens_for_tokens(
            env: Env,
            amount_in: i128,
            _amount_out_min: i128,
            _path: soroban_sdk::Vec<Address>,
            _to: Address,
            _deadline: u64,
        ) -> soroban_sdk::Vec<i128> {
            // Mock: 1 USDC = 2 XLM
            let mut result = soroban_sdk::Vec::new(&env);
            result.push_back(amount_in);
            result.push_back(amount_in * 2);
            result
        }

        pub fn add_liquidity(
            _env: Env,
            _token_a: Address,
            _token_b: Address,
            _amount_a_desired: i128,
            _amount_b_desired: i128,
            _amount_a_min: i128,
            _amount_b_min: i128,
            _to: Address,
            _deadline: u64,
        ) -> (i128, i128, i128) {
            // Mock: Returns (amount_a, amount_b, lp_shares)
            // For simplicity, lp_shares = amount_a
            (100, 200, 100) 
        }
    }

    #[test]
    fn test_initialize_gold_trustline() {
        let env = Env::default();
        let usdc = Address::generate(&env);
        let contract_id = env.register(SmasageYieldRouter, ());
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin, &usdc);
        client.init_gold_trustline(&admin, &5_000_000);

        let (asset_code, asset_issuer) = client.get_gold_asset();
        assert_eq!(asset_code, symbol_short!("XAUT"));
        assert_eq!(
            asset_issuer,
            String::from_str(&env, "GCRLXTLD7XIRXWXV2PDCC74O5TUUKN3OODJAM6TWVE4AIRNMGQJK3KWQ")
        );
        assert!(client.is_gold_trustline_ready());
        assert_eq!(client.get_gold_reserve_stroops(), 5_000_000);
    }

    #[test]
    fn test_deposit_withdraw() {
        let env = Env::default();
        let usdc = Address::generate(&env);
        let contract_id = env.register(SmasageYieldRouter, ());
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let admin = Address::generate(&env);
        
        env.mock_all_auths();

        client.initialize(&admin, &usdc);

        // 60% Blend, 30% LP, 10% Gold (mocked conceptually)
<<<<<<< HEAD
        client.deposit(&user, &1000, &60, &30);
        // 60% Blend, 30% LP, 10% Gold
        client.deposit(&user, &1000, &60, &30, &10);
=======
        // We haven't configured Soroswap yet, so lp_percentage must be 0 or it will panic
        client.deposit(&user, &1000, &60, &0);
>>>>>>> 46ab13d (Implement Soroswap LP Integration (Issue 2.3) with 50/50 split logic and LP share tracking)
        
        assert_eq!(client.get_balance(&user), 1000);
        assert_eq!(client.get_gold_balance(&user), 100);
        assert_eq!(client.get_lp_shares(&user), 300);
        
        client.withdraw(&user, &500);
        assert_eq!(client.get_balance(&user), 500);
    }

    #[test]
<<<<<<< HEAD
    fn test_withdraw_unwinds_blend_and_lp() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SmasageYieldRouter);
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        // Deposit with 60% to Blend, 30% to LP, 10% to Gold
        client.deposit(&user, &1000, &60, &30, &10);
        
        // Verify allocations
        assert_eq!(client.get_balance(&user), 1000);
        assert_eq!(client.get_gold_balance(&user), 100);
        assert_eq!(client.get_lp_shares(&user), 300);
        
        // Withdraw full amount - should unwind from all sources
        client.withdraw(&user, &1000);
        assert_eq!(client.get_balance(&user), 0);
        // Note: Gold and LP remain because withdrawal priority uses USDC first
        // In a real scenario, these would be unwound as needed
        assert_eq!(client.get_gold_balance(&user), 100);
        assert_eq!(client.get_lp_shares(&user), 300);
    }

    #[test]
    fn test_gold_allocation_tracking() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SmasageYieldRouter);
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        // Deposit with 20% Gold allocation
        client.deposit(&user, &2000, &50, &30, &20);
        
        assert_eq!(client.get_gold_balance(&user), 400);
        
        // Partial withdrawal shouldn't affect gold unless needed
        client.withdraw(&user, &500);
        // Gold should remain intact if USDC balance is sufficient
        assert_eq!(client.get_gold_balance(&user), 400);
=======
    fn test_soroswap_lp_integration() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let usdc_addr = Address::generate(&env);
        let xlm_addr = Address::generate(&env);

        let router_id = env.register(MockSoroswapRouter, ());
        
        let contract_id = env.register(SmasageYieldRouter, ());
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        client.initialize(&admin, &usdc_addr);
        client.configure_soroswap(&admin, &router_id, &xlm_addr);

        // Deposit 1000 USDC, 50% to LP
        // Logic should:
        // 1. Take 500 USDC for LP.
        // 2. Swap 250 USDC for XLM -> Mock returns 500 XLM.
        // 3. Add liquidity with 250 USDC and 500 XLM -> Mock returns 100 LP shares (hardcoded in mock).
        client.deposit(&user, &1000, &0, &50);

        assert_eq!(client.get_balance(&user), 1000);
        assert_eq!(client.get_lp_shares(&user), 100); 
>>>>>>> 46ab13d (Implement Soroswap LP Integration (Issue 2.3) with 50/50 split logic and LP share tracking)
    }
}
