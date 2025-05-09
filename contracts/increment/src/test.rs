#![cfg(test)]

use super::{ExchangeRateEURUSD, ExchangeRateEURUSDClient};
use soroban_sdk::{
    testutils::{Events, Logs},
    Env,
    String
};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExchangeRateEURUSD);
    let client = ExchangeRateEURUSDClient::new(&env, &contract_id);
    client.write(&String::from_str(&env, "TEST"));
    assert_eq!(client.get_latest(), String::from_str(&env, "TEST"));


  
    std::println!("{}", env.logs().all().join("\n"));
}
