#![cfg(test)]

use super::*;
use soroban_sdk::{Env, U256};

#[test]
fn test_valid_scalar_below_modulus() {
    let env = Env::default();
    let contract_id = env.register(Verifier, ());
    let client = VerifierClient::new(&env, &contract_id);

    // A small valid scalar (well below BN254 modulus)
    let val = U256::from_u128(&env, 42);
    assert!(client.check(&val));
}

#[test]
fn test_invalid_scalar_above_modulus() {
    let env = Env::default();
    let contract_id = env.register(Verifier, ());
    let client = VerifierClient::new(&env, &contract_id);

    // U256::MAX is way above BN254 modulus — must be rejected
    let val = U256::from_be_bytes(&env, &[0xff; 32]);
    assert!(!client.check(&val));
}

#[test]
fn test_zero_is_valid_scalar() {
    let env = Env::default();
    let contract_id = env.register(Verifier, ());
    let client = VerifierClient::new(&env, &contract_id);

    let val = U256::from_u128(&env, 0);
    assert!(client.check(&val));
}

#[test]
fn test_modulus_itself_is_invalid() {
    let env = Env::default();
    let contract_id = env.register(Verifier, ());
    let client = VerifierClient::new(&env, &contract_id);

    // BN254 modulus exactly — must fail (valid range is 0..modulus-1)
    let modulus_bytes: [u8; 32] = [
        0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58,
        0x5d, 0x97, 0x81, 0x6a, 0x91, 0x68, 0x71, 0xca, 0x8d, 0x3c, 0x20, 0x8c, 0x16, 0xd8, 0x7c,
        0xfd, 0x47,
    ];
    let val = U256::from_be_bytes(&env, &modulus_bytes);
    assert!(!client.check(&val));
}
