// Simple benchmark placeholder for Merkle tree generation performance
use shared::{AuditLog, generate_merkle_root};
use std::time::Instant;

fn main() {
    println!("Memulai benchmark performa generasi Merkle Tree...");
    let mut logs = Vec::new();
    
    for i in 0..10_000 {
        let mut log = AuditLog {
            id: None,
            service: "benchmark-service".to_string(),
            user_id: format!("user_{}", i),
            amount: (i as f64) * 1.5,
            timestamp: "2026-07-24T12:00:00Z".to_string(),
            hash: None,
            batch_id: Some("bench_batch".to_string()),
            event_type: Some("BENCHMARK".to_string()),
            actor_id: Some("bench-agent".to_string()),
            signature: None,
            public_key: None,
        };
        log.hash = Some(log.compute_hash());
        logs.push(log);
    }

    let start = Instant::now();
    let root = generate_merkle_root(&logs);
    let elapsed = start.elapsed();

    println!("✅ Selesai memproses 10,000 log audit!");
    println!("   Merkle Root: {}", root);
    println!("   Waktu Komputasi: {:?}", elapsed);
}
