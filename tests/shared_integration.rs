use shared::{AuditLog, generate_merkle_root};

#[test]
fn hash_and_merkle_root_basic() {
    let mut a = AuditLog {
        id: None, service: "svc".to_string(), user_id: "user1".to_string(), amount: 1.23,
        timestamp: "t1".to_string(), hash: None, batch_id: None, event_type: Some("TRANSFER".to_string()),
        actor_id: Some("actor1".to_string()), signature: None, public_key: None,
    };
    a.hash = Some(a.compute_hash());

    let mut b = AuditLog {
        id: None, service: "svc".to_string(), user_id: "user2".to_string(), amount: 4.56,
        timestamp: "t2".to_string(), hash: None, batch_id: None, event_type: Some("WITHDRAW".to_string()),
        actor_id: Some("actor2".to_string()), signature: None, public_key: None,
    };
    b.hash = Some(b.compute_hash());

    let root = generate_merkle_root(&[a, b]);
    assert!(!root.is_empty());
    assert_eq!(root.len(), 64); // hex-encoded 32-byte root
}

#[test]
fn tamper_detection_field_level() {
    let mut original = AuditLog {
        id: None, service: "auth-service".to_string(), user_id: "U100".to_string(), amount: 0.0,
        timestamp: "2026-07-24T10:00:00Z".to_string(), hash: None, batch_id: Some("b1".to_string()),
        event_type: Some("LOGIN".to_string()), actor_id: Some("user-agent-1".to_string()),
        signature: None, public_key: None,
    };
    original.hash = Some(original.compute_hash());
    let root1 = generate_merkle_root(&[original.clone()]);

    let mut tampered = original.clone();
    tampered.event_type = Some("ADMIN_ELEVATE".to_string());
    tampered.hash = Some(tampered.compute_hash());
    let root2 = generate_merkle_root(&[tampered]);

    assert_ne!(root1, root2, "Changing event_type must alter computed Merkle root");
}

#[test]
fn empty_logs_merkle_root() {
    let root = generate_merkle_root(&[]);
    assert_eq!(root.len(), 64);
}
