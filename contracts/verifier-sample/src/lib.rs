#![no_std]
use soroban_sdk::{contract, contractimpl, Env, U256};
use soroban_zk_std::ZkEnv;

#[contract]
pub struct Verifier;

#[contractimpl]
impl Verifier {
    pub fn check(env: Env, input: U256) -> bool {
        env.is_bn254_scalar(input)
    }
}

mod test;
