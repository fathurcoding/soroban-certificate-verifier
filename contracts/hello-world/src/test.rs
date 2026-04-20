#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, String};

#[test]
fn test_register_and_get() {
    let env = Env::default();
    let contract_id = env.register(VeriChain, ());
    let client = VeriChainClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let hash = Bytes::from_slice(&env, b"my_hash_123");
    let title = String::from_slice(&env, "Degree Certificate");

    let success = client.register_certificate(&owner, &hash, &title);
    assert_eq!(success, true);

    let (ret_owner, _timestamp, ret_title) = client.get_certificate_record(&hash);
    assert_eq!(ret_owner, owner);
    assert_eq!(ret_title, title);

    let is_valid = client.validate_certificate(&hash);
    assert_eq!(is_valid, true);

    let user_certs = client.get_user_certificates(&owner);
    assert_eq!(user_certs.len(), 1);
    assert_eq!(user_certs.get(0).unwrap(), hash);
}

#[test]
#[should_panic(expected = "Certificate hash already registered.")]
fn test_duplicate_registration() {
    let env = Env::default();
    let contract_id = env.register(VeriChain, ());
    let client = VeriChainClient::new(&env, &contract_id);

    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let hash = Bytes::from_slice(&env, b"duplicate_hash");
    let title = String::from_slice(&env, "First Title");

    client.register_certificate(&owner1, &hash, &title);

    // This should panic
    let title2 = String::from_slice(&env, "Second Title");
    client.register_certificate(&owner2, &hash, &title2);
}

#[test]
#[should_panic(expected = "Title must be between 1 and 50 characters.")]
fn test_invalid_title() {
    let env = Env::default();
    let contract_id = env.register(VeriChain, ());
    let client = VeriChainClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let hash = Bytes::from_slice(&env, b"hash");
    let title = String::from_slice(&env, "");

    // This should panic
    client.register_certificate(&owner, &hash, &title);
}
