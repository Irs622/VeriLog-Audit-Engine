use rusqlite::{Connection, Result};

pub fn init_db_from_connection(conn: &Connection) -> Result<()> {
    // Hardened SQLite Configuration
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            service TEXT NOT NULL,
            user_id TEXT NOT NULL,
            amount REAL NOT NULL,
            timestamp TEXT NOT NULL,
            hash TEXT NOT NULL,
            batch_id TEXT,
            event_type TEXT DEFAULT 'SYSTEM',
            actor_id TEXT DEFAULT 'system-agent',
            signature TEXT,
            public_key TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS batches (
            id TEXT PRIMARY KEY,
            merkle_root TEXT NOT NULL,
            service_id TEXT NOT NULL,
            chain_status TEXT DEFAULT 'PENDING',
            verify_status TEXT DEFAULT 'UNVERIFIED',
            tx_hash TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );"
    )?;

    // Auto-Migration for existing databases lacking signature/public_key
    let _ = conn.execute("ALTER TABLE audit_logs ADD COLUMN signature TEXT;", []);
    let _ = conn.execute("ALTER TABLE audit_logs ADD COLUMN public_key TEXT;", []);

    Ok(())
}

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    init_db_from_connection(&conn)?;
    Ok(conn)
}
