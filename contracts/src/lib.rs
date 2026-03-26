#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

// Issue 2: Smart Contract - Stellar Path Payments & Yield Allocation (Blend Integration)

#[contracttype]
pub enum DataKey {
    Admin,
    UserBalance(Address),
    TotalDeposits,
    GoldAssetCode,
    GoldAssetIssuer,
    GoldTrustlineReady,
    GoldTrustlineReserveStroops,
}

const CANONICAL_GOLD_ASSET_CODE: Symbol = symbol_short!("XAUT");
const CANONICAL_GOLD_ASSET_ISSUER: &str = "GCRLXTLD7XIRXWXV2PDCC74O5TUUKN3OODJAM6TWVE4AIRNMGQJK3KWQ";
const TRUSTLINE_BASE_RESERVE_STROOPS: i128 = 5_000_000;

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
    /// In a real implementation, this would handle token transfers and issue calls to the Blend Protocol.
    pub fn deposit(env: Env, from: Address, amount: i128, blend_percentage: u32, lp_percentage: u32) {
        from.require_auth();
        assert!(blend_percentage + lp_percentage <= 100, "Allocation exceeds 100%");
        
        let mut balance: i128 = env.storage().persistent().get(&DataKey::UserBalance(from.clone())).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&DataKey::UserBalance(from.clone()), &balance);
        
        // Mock: Here we would route `blend_percentage` to the Blend protocol
        // Mock: Here we would route `lp_percentage` to Soroswap Pool
    }

    pub fn withdraw(env: Env, to: Address, amount: i128) {
        to.require_auth();
        let mut balance: i128 = env.storage().persistent().get(&DataKey::UserBalance(to.clone())).unwrap_or(0);
        assert!(balance >= amount, "Insufficient balance");
        balance -= amount;
        env.storage().persistent().set(&DataKey::UserBalance(to.clone()), &balance);
        
        // Mock: Here we would break LP positions and retrieve from Blend Protocol
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&DataKey::UserBalance(user)).unwrap_or(0)
    }
}

// Basic Test Mock
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_initialize_gold_trustline() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SmasageYieldRouter);
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);
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
        let contract_id = env.register_contract(None, SmasageYieldRouter);
        let client = SmasageYieldRouterClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let admin = Address::generate(&env);
        
        env.mock_all_auths();

        client.initialize(&admin);

        // 60% Blend, 30% LP, 10% Gold (mocked conceptually)
        client.deposit(&user, &1000, &60, &30);
        
        assert_eq!(client.get_balance(&user), 1000);
        
        client.withdraw(&user, &500);
        assert_eq!(client.get_balance(&user), 500);
    }
}
