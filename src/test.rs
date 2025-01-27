#![cfg(test)]
extern crate std;
use std::println;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, vec, Address, Env};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

#[test]
fn test_simple() {
    let env = Env::default();
    env.mock_all_auths();

    let token_admin = Address::generate(&env);
    let address1 = Address::generate(&env);
    let address2 = Address::generate(&env);
    let address3 = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    token_admin_client.mint(&address1, &500);
    token_admin_client.mint(&address2, &500);
    token_admin_client.mint(&address3, &500);
    
    let client = SorobatchContractClient::new(&env, &env.register(SorobatchContract, ()));


    let result = client.batch_transfer(&address1, &vec![
        &env,
        PaymentDetails {
            token: token.address.clone(),
            amount: 100,
            destination: address2.clone(),
        },
        PaymentDetails {
            token: token.address.clone(),
            amount: 1000,
            destination: address2.clone(),
        }
    ]);
    assert_eq!(token.balance(&address1), 400);
    assert_eq!(token.balance(&address2), 600);
    assert_eq!(result, vec![&env, true, false]);
}


mod wasm_contract {
    //todo: why is this needed?
    soroban_sdk::contractimport!(
        file = "target/wasm32-unknown-unknown/release/sorobatch.wasm"
    );
}

#[test]
fn test_exhaust_token() {
    let env = Env::default();
    env.mock_all_auths();

    // let client = SorobatchContractClient::new(&env, &env.register(SorobatchContract, ()));
    let client = SorobatchContractClient::new(&env,&env.register(wasm_contract::WASM, ()));

    let token_admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    let faucet = Address::generate(&env);
    token_admin_client.mint(&faucet, &(i64::MAX as i128));

    for i in 48..51 {
        println!("testings with {} addresses", i);
        env.cost_estimate().budget().reset_default();
        
        let mut addresses = Vec::new(&env);
        for _ in 0..i {
            addresses.push_back(Address::generate(&env));
        }
    
        let mut payments = Vec::new(&env);
        for address in addresses {
            token_admin_client.mint(&address, &1);
            payments.push_back(PaymentDetails {
                token: token.address.clone(),
                amount: 1,
                destination: address.clone(),
            });
        }
    
        let result = client.batch_transfer(&faucet, &payments);
        assert_eq!(result.len(), i);
        println!("Instructions: {:#?}", &env.cost_estimate().resources());
    }
}
