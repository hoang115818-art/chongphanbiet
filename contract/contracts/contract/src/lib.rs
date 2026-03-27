#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol
};

#[contract]
pub struct UnityContract;

// Storage Keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Total,
    Message,
    User(Address),
}

#[contractimpl]
impl UnityContract {

    // ===== Initialize Contract (set admin) =====
    pub fn initialize(env: Env, admin: Address, message: String) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Total, &0u32);
        env.storage().instance().set(&DataKey::Message, &message);
    }

    // ===== User Commitment =====
    pub fn commit(env: Env, user: Address) {
        user.require_auth();

        let user_key = DataKey::User(user.clone());

        if !env.storage().persistent().has(&user_key) {
            env.storage().persistent().set(&user_key, &true);

            let mut total: u32 = env
                .storage()
                .instance()
                .get(&DataKey::Total)
                .unwrap_or(0);

            total += 1;

            env.storage().instance().set(&DataKey::Total, &total);
        }
    }

    // ===== Get Total Commitments =====
    pub fn get_total(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Total)
            .unwrap_or(0)
    }

    // ===== Set Unity Message (Admin Only) =====
    pub fn set_message(env: Env, admin: Address, message: String) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");

        if admin != stored_admin {
            panic!("Not authorized");
        }

        env.storage().instance().set(&DataKey::Message, &message);
    }

    // ===== Get Current Message =====
    pub fn get_message(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Message)
            .unwrap()
    }
}