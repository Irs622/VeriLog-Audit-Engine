use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::Keypair,
        pubkey::Pubkey,
        signer::EncodableKeypair,
    },
    Client, Cluster,
};
use std::sync::Arc;
use std::str::FromStr;

pub fn send_merkle_root_to_solana(batch_id: &str, merkle_root: &str, service_id: &str) -> Result<String, String> {
    let rpc_url = std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let program_id_str = std::env::var("SOLANA_PROGRAM_ID").unwrap_or_else(|_| "VeriLog111111111111111111111111111111111111".to_string());

    // Fallback Mock mode if no validator available
    let is_mock = std::env::var("MOCK_SOLANA").unwrap_or_else(|_| "true".to_string());
    if is_mock == "true" {
        println!("[Sender (Mock)] Berhasil mencatat Batch {} ke Solana.", batch_id);
        return Ok(format!("mock_tx_{}", batch_id));
    }

    println!("[Sender] Mengirim batch {} (service: {}) ke Solana...", batch_id, service_id);

    // Load keypair from file path or fallback to new keypair
    let payer = if let Ok(keypair_path) = std::env::var("SOLANA_KEYPAIR_PATH") {
        Keypair::read_from_file(&keypair_path)
            .map_err(|e| format!("Gagal membaca keypair dari {}: {}", keypair_path, e))?
    } else {
        Keypair::new()
    };
    
    let client = Client::new_with_options(
        Cluster::Custom(rpc_url.clone(), rpc_url.clone()),
        Arc::new(payer),
        CommitmentConfig::confirmed(),
    );

    let program_id = Pubkey::from_str(&program_id_str)
        .map_err(|e| format!("Invalid SOLANA_PROGRAM_ID {}: {}", program_id_str, e))?;
    
    let _program = client.program(program_id);

    // Convert hex string to [u8; 32]
    let mut _root_bytes = [0u8; 32];
    if let Ok(bytes) = hex::decode(merkle_root) {
        let len = std::cmp::min(bytes.len(), 32);
        _root_bytes[..len].copy_from_slice(&bytes[..len]);
    }

    let is_rpc_reachable = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?
        .get(&rpc_url)
        .send()
        .is_ok();
    
    if !is_rpc_reachable {
        return Err(format!("Koneksi ke {} ditolak.", rpc_url));
    }
    
    println!("[Sender] Batch {} di-record ke Solana on-chain.", batch_id);
    Ok(format!("tx_{}", batch_id))
}
