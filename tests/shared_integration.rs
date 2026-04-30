use shared::{AuditLog, generate_merkle_root};

#[test]
fn hash_and_merkle_root_basic() {
    let mut a = AuditLog { id: None, service: "svc".to_string(), user_id: "user1".to_string(), amount: 1.23, timestamp: "t1".to_string(), hash: None, batch_id: None, event_type: None, actor_id: None };
    a.hash = Some(a.compute_hash());

    let mut b = AuditLog { id: None, service: "svc".to_string(), user_id: "user2".to_string(), amount: 4.56, timestamp: "t2".to_string(), hash: None, batch_id: None, event_type: None, actor_id: None };
    b.hash = Some(b.compute_hash());

    let root = generate_merkle_root(&[a, b]);
    assert!(!root.is_empty());
    assert_eq!(root.len(), 64); // hex-encoded 32-byte root
}
