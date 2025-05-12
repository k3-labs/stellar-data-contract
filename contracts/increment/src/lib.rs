#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contract]
pub struct ExchangeRateEURUSD;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Data,
}

#[contractimpl]
impl ExchangeRateEURUSD {
    pub fn write(env: Env, value: String) {
        env.storage().persistent().set(&DataKey::Data, &value);
        env.events().publish((symbol_short!("update"), value.clone()), value.clone());
    }

    pub fn get_latest(env: Env) -> String {
        env.storage().persistent()
            .get(&DataKey::Data)
            .unwrap_or(String::from_str(&env, "None"))
    }
}

mod test;