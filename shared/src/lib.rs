pub mod db;

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use rs_merkle::{MerkleTree, Hasher};

// Standard API Contract
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
    pub meta: Option<ApiMeta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorData {
    pub code: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub status: String,
    pub error: ApiErrorData,
}

#[derive(Clone)]
pub struct Keccak256Algorithm {}

impl Hasher for Keccak256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let mut res = [0u8; 32];
        res.copy_from_slice(hasher.finalize().as_slice());
        res
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLog {
    pub id: Option<i64>,
    pub service: String,
    pub user_id: String,
    pub amount: f64,
    pub timestamp: String,
    pub hash: Option<String>,
    pub batch_id: Option<String>,
    pub event_type: Option<String>,
    pub actor_id: Option<String>,
    pub signature: Option<String>,
    pub public_key: Option<String>,
}

impl AuditLog {
    pub fn compute_hash(&self) -> String {
        let event_type = self.event_type.as_deref().unwrap_or("SYSTEM");
        let actor_id = self.actor_id.as_deref().unwrap_or("system-agent");
        let payload = format!(
            "{}|{}|{}|{}|{}|{}",
            self.service, self.user_id, self.amount, self.timestamp, event_type, actor_id
        );
        let hash_bytes = Keccak256Algorithm::hash(payload.as_bytes());
        hex::encode(hash_bytes)
    }

    pub fn verify_signature(&self) -> bool {
        match (&self.signature, &self.public_key) {
            (Some(sig_hex), Some(pk_hex)) => {
                let sig_bytes = match hex::decode(sig_hex) {
                    Ok(b) if b.len() == 64 => {
                        let mut arr = [0u8; 64];
                        arr.copy_from_slice(&b);
                        arr
                    }
                    _ => return false,
                };
                let pk_bytes = match hex::decode(pk_hex) {
                    Ok(b) if b.len() == 32 => {
                        let mut arr = [0u8; 32];
                        arr.copy_from_slice(&b);
                        arr
                    }
                    _ => return false,
                };

                let verifying_key = match ed25519_dalek::VerifyingKey::from_bytes(&pk_bytes) {
                    Ok(k) => k,
                    Err(_) => return false,
                };
                let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes);
                let hash_str = self.hash.clone().unwrap_or_else(|| self.compute_hash());

                verifying_key.verify_strict(hash_str.as_bytes(), &signature).is_ok()
            }
            _ => true, // Signature optional for backwards compatibility
        }
    }
}

pub fn generate_merkle_root(logs: &[AuditLog]) -> String {
    if logs.is_empty() {
        let empty_hash = Keccak256Algorithm::hash(b"");
        return hex::encode(empty_hash);
    }

    let mut leaves: Vec<[u8; 32]> = Vec::new();
    for log in logs {
        if let Some(h) = &log.hash {
            if let Ok(bytes) = hex::decode(h) {
                let mut leaf = [0u8; 32];
                if bytes.len() >= 32 {
                    leaf.copy_from_slice(&bytes[..32]);
                    leaves.push(leaf);
                }
            }
        }
    }
    
    let merkle_tree = MerkleTree::<Keccak256Algorithm>::from_leaves(&leaves);
    
    if let Some(root) = merkle_tree.root() {
        hex::encode(root)
    } else {
        hex::encode(Keccak256Algorithm::hash(b""))
    }
}
