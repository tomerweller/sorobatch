#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[contract]
pub struct SorobatchContract;

#[derive(Clone)]
#[contracttype]
pub struct PaymentDetails {
    pub token: Address,
    pub amount: i128,
    pub destination: Address
}

#[contractimpl]
impl SorobatchContract {
    pub fn batch_transfer(
        env: Env,
        from: Address,
        payments: Vec<PaymentDetails>,
    ) -> Vec<bool> {
        from.require_auth();
        let mut results = Vec::new(&env);
        for recipient in payments {
            let success = token::Client::new(&env, &recipient.token).try_transfer(&from, &recipient.destination, &recipient.amount).is_ok();
            results.push_back(success);
        }
        results
    }

}

mod test;
