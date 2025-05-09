#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};
use soroban_sdk::String;

const DATA: Symbol = symbol_short!("DATA");
#[contract]
pub struct ExchangeRateEURUSD;

#[contractimpl]
impl ExchangeRateEURUSD {
    pub fn write(env: Env, value: String) {
        env.storage().persistent().set(&DATA, &value);
    }

    pub fn get_latest(env: Env) -> String{
        env.storage().persistent().get(&DATA).unwrap_or(String::from_str(&env, "None"))
    }
}

mod test;
