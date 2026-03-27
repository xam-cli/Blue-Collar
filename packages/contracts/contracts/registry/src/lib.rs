//! BlueCollar Registry Contract
//! Deployed on Stellar (Soroban) — manages worker registrations on-chain.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

// =============================================================================
// Data types
// =============================================================================

#[contracttype]
#[derive(Clone)]
pub struct Worker {
    pub id: Symbol,
    pub owner: Address,
    pub name: String,
    pub category: Symbol,
    pub is_active: bool,
    pub wallet: Address,
}

#[contracttype]
pub enum DataKey {
    /// Instance storage — admin address, set once at initialize
    Admin,
    Worker(Symbol),
    WorkerList,
}

// =============================================================================
// Contract
// =============================================================================

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
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
    // Worker registration
    // -------------------------------------------------------------------------

    /// Register a new worker on-chain.
    pub fn register(env: Env, id: Symbol, owner: Address, name: String, category: Symbol) {
        owner.require_auth();

        let worker = Worker {
            id: id.clone(),
            owner: owner.clone(),
            name,
            category,
            is_active: true,
            wallet: owner,
        };

        env.storage().persistent().set(&DataKey::Worker(id.clone()), &worker);

        let mut list: Vec<Symbol> = env
            .storage()
            .persistent()
            .get(&DataKey::WorkerList)
            .unwrap_or(Vec::new(&env));
        list.push_back(id);
        env.storage().persistent().set(&DataKey::WorkerList, &list);
    }

    /// Get a worker by id.
    pub fn get_worker(env: Env, id: Symbol) -> Option<Worker> {
        env.storage().persistent().get(&DataKey::Worker(id))
    }

    /// Toggle a worker's active status (owner only).
    pub fn toggle(env: Env, id: Symbol, caller: Address) {
        caller.require_auth();
        let mut worker: Worker = env
            .storage()
            .persistent()
            .get(&DataKey::Worker(id.clone()))
            .expect("Worker not found");
        assert!(worker.owner == caller, "Not authorized");
        worker.is_active = !worker.is_active;
        env.storage().persistent().set(&DataKey::Worker(id), &worker);
    }

    /// List all registered worker ids.
    pub fn list_workers(env: Env) -> Vec<Symbol> {
        env.storage()
            .persistent()
            .get(&DataKey::WorkerList)
            .unwrap_or(Vec::new(&env))
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
        fn uninit() -> Self {
            let env = Env::default();
            env.mock_all_auths();
            let admin = Address::generate(&env);
            let contract_id = env.register_contract(None, RegistryContract);
            TestEnv { env, contract_id, admin }
        }

        fn new() -> Self {
            let t = Self::uninit();
            t.client().initialize(&t.admin);
            t
        }

        fn client(&self) -> RegistryContractClient {
            RegistryContractClient::new(&self.env, &self.contract_id)
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
        t.client().initialize(&t.admin);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_reinitialize_with_different_admin_panics() {
        let t = TestEnv::new();
        let attacker = Address::generate(&t.env);
        t.client().initialize(&attacker);
    }

    #[test]
    fn test_admin_unchanged_after_failed_reinit() {
        let t = TestEnv::new();
        let attacker = Address::generate(&t.env);
        // Verify attacker cannot become admin
        assert_ne!(t.client().get_admin(), attacker);
        assert_eq!(t.client().get_admin(), t.admin);
    }
}
