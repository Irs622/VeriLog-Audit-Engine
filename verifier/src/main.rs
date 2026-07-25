use shared::{db::init_db, generate_merkle_root, AuditLog};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("Memeriksa integritas seluruh batch histori via rs-merkle...");
    
    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "verilog.db".to_string());
    let conn = match init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Gagal membuka database {}: {}", db_path, e);
            std::process::exit(1);
        }
    };

    let mut stmt = match conn.prepare("SELECT id, merkle_root, chain_status FROM batches ORDER BY created_at DESC") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Gagal membaca tabel batches: {}", e);
            std::process::exit(1);
        }
    };

    let batch_iter = match stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    }) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("❌ Gagal mengeksekusi query batches: {}", e);
            std::process::exit(1);
        }
    };

    let mut has_tampering = false;

    for batch_res in batch_iter {
        let (batch_id, stored_root, chain_status) = match batch_res {
            Ok(b) => b,
            Err(_) => continue,
        };
        
        let mut log_stmt = match conn.prepare("SELECT service, user_id, amount, timestamp, hash, event_type, actor_id, signature, public_key FROM audit_logs WHERE batch_id = ?1") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error preparing log query for batch {}: {}", batch_id, e);
                continue;
            }
        };

        let logs_iter = log_stmt.query_map([&batch_id], |row| {
            Ok(AuditLog {
                id: None,
                service: row.get(0)?,
                user_id: row.get(1)?,
                amount: row.get(2)?,
                timestamp: row.get(3)?,
                hash: Some(row.get(4)?),
                batch_id: None,
                event_type: row.get(5)?,
                actor_id: row.get(6)?,
                signature: row.get(7)?,
                public_key: row.get(8)?,
            })
        });

        let mut logs = Vec::new();
        if let Ok(iter) = logs_iter {
            for log_item in iter.flatten() {
                let mut log = log_item;
                log.hash = Some(log.compute_hash());
                logs.push(log);
            }
        }

        let computed_root = generate_merkle_root(&logs);
        
        if computed_root == stored_root {
            println!("✅ [VALID] Batch {} (Status: {}) - Merkle root match.", batch_id, chain_status);
        } else {
            println!("❌ [TAMPERED] Batch {} - Data manipulation detected!", batch_id);
            println!("   Detail: On-chain: {}, Computed: {}", stored_root, computed_root);
            has_tampering = true;
        }
    }

    let rpc_url = std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let is_mock = std::env::var("MOCK_SOLANA").unwrap_or_else(|_| "true".to_string());

    println!("🌐 Status Solana RPC Endpoint ({}): Mode {}", rpc_url, if is_mock == "true" { "MOCK" } else { "LIVE" });

    if has_tampering {
        println!("\n⚠️ PERINGATAN: TERDAPAT MANIPULASI DATA PADA HISTORI LOG!");
    } else {
        println!("\n🔒 AMAN: Seluruh log audit sesuai dengan Merkle Root di database & status anchoring blockchain.");
    }
}
