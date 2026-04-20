#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertRecord {
    pub owner: Address,
    pub timestamp: u64,
    pub title: String,
}

#[contracttype]
pub enum DataKey {
    Record(Bytes),      // Maps Hash -> CertRecord
    UserCerts(Address), // Maps Address -> Vec<Bytes>
}

#[contract]
pub struct VeriChain;

#[contractimpl]
impl VeriChain {
    pub fn register_certificate(env: Env, owner: Address, hash: Bytes, title: String) -> bool {
        owner.require_auth();

        let title_len = title.len();
        if title_len == 0 || title_len > 50 {
            panic!("Title must be between 1 and 50 characters.");
        }

        let record_key = DataKey::Record(hash.clone());
        if env.storage().persistent().has(&record_key) {
            panic!("Certificate hash already registered.");
        }

        let timestamp = env.ledger().timestamp();
        let record = CertRecord {
            owner: owner.clone(),
            timestamp,
            title,
        };
        env.storage().persistent().set(&record_key, &record);

        let user_key = DataKey::UserCerts(owner.clone());
        let mut user_certs: Vec<Bytes> = env.storage().persistent().get(&user_key).unwrap_or_else(|| Vec::new(&env));
        user_certs.push_back(hash.clone());
        env.storage().persistent().set(&user_key, &user_certs);

        env.events().publish(
            (symbol_short!("register"), hash),
            owner,
        );

        true
    }

    pub fn validate_certificate(env: Env, hash: Bytes) -> bool {
        let record_key = DataKey::Record(hash);
        env.storage().persistent().has(&record_key)
    }

    pub fn get_certificate_record(env: Env, hash: Bytes) -> (Address, u64, String) {
        let record_key = DataKey::Record(hash);
        let record: CertRecord = env.storage().persistent().get(&record_key).unwrap_or_else(|| panic!("Certificate not found."));
        (record.owner, record.timestamp, record.title)
    }

    pub fn get_user_certificates(env: Env, address: Address) -> Vec<Bytes> {
        let user_key = DataKey::UserCerts(address);
        env.storage().persistent().get(&user_key).unwrap_or_else(|| Vec::new(&env))
    }
}

mod test;
