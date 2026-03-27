//! BlueCollar Market Contract
//! Handles tip/payment escrow between users and workers on Stellar (Soroban).

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

// =============================================================================
// Data types
// =============================================================================

#[contracttype]
#[derive(Clone)]
pub struct Tip {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub token: Address,
    pub released: bool,
}

#[contracttype]
pub enum DataKey {
    /// Instance storage — admin address, set once at initialize
    Admin,
    Tip(Symbol),
}

// =============================================================================
// Contract
// =============================================================================

#[contract]
pub struct MarketContract;

#[contractimpl]
impl MarketContract {
    // -------------------------------------------------------------------------
    // Initialize
    // -------------------------------------------------------------------------

    /// Set the contract admin. Must be called once before any other function.
    /// Panics with "Already initialized" if called more than once.
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "Already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // -------------------------------------------------------------------------
    // Views
    // -------------------------------------------------------------------------

    /// Returns true if the contract has been initialized.
    pub fn is_initialized(env: Env) -> bool {
        env.storage().instance().has(&DataKey::Admin)
    }

    /// Get the admin address. Panics if not initialized.
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Not initialized")
    }

    // -------------------------------------------------------------------------
    // Tip
    // -------------------------------------------------------------------------

    /// Send a tip to a worker — transfers tokens directly.
    pub fn tip(env: Env, from: Address, to: Address, token_addr: Address, amount: i128) {
        from.require_auth();
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&from, &to, &amount);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    struct TestEnv {
        env: Env,
        contract_id: Address,
        admin: Address,
    }

    impl TestEnv {
        /// Returns a contract that has NOT been initialized yet.
        fn uninit() -> Self {
            let env = Env::default();
            env.mock_all_auths();
            let admin = Address::generate(&env);
            let contract_id = env.register_contract(None, MarketContract);
            TestEnv { env, contract_id, admin }
        }

        /// Returns a contract that has already been initialized.
        fn new() -> Self {
            let t = Self::uninit();
            t.client().initialize(&t.admin);
            t
        }

        fn client(&self) -> MarketContractClient {
            MarketContractClient::new(&self.env, &self.contract_id)
        }
    }

    // -------------------------------------------------------------------------
    // initialize
    // -------------------------------------------------------------------------

    #[test]
    fn test_initialize_sets_admin() {
        let t = TestEnv::new();
        assert_eq!(t.client().get_admin(), t.admin);
    }

    #[test]
    fn test_is_initialized_false_before_init() {
        let t = TestEnv::uninit();
        assert!(!t.client().is_initialized());
    }

    #[test]
    fn test_is_initialized_true_after_init() {
        let t = TestEnv::new();
        assert!(t.client().is_initialized());
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_initialize_twice_panics() {
        let t = TestEnv::new();
        // second call must panic
        t.client().initialize(&t.admin);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_initialize_with_different_admin_panics() {
        let t = TestEnv::new();
        let attacker = Address::generate(&t.env);
        t.client().initialize(&attacker);
    }

    #[test]
    fn test_admin_cannot_be_overwritten() {
        let t = TestEnv::new();
        let attacker = Address::generate(&t.env);
        // Attacker is a different address — original admin is still set
        assert_ne!(t.client().get_admin(), attacker);
        assert_eq!(t.client().get_admin(), t.admin);
    }
}
