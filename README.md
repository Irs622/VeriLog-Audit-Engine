# VeriLog Audit Engine (Rust)

VeriLog is a **decentralized, tamper-evident audit logging system** designed for distributed and microservices-based architectures. It ensures log integrity through cryptographic verification and optional blockchain anchoring.

The system is implemented in **Rust** to achieve high performance, memory safety, and predictable concurrency behavior.

---

## Overview

Traditional logging systems are vulnerable to **log tampering**, especially in distributed environments where trust boundaries are unclear. VeriLog addresses this by:

- Enforcing **append-only logging**
- Using **cryptographic hashing (hash chaining / Merkle structures)**
- Providing **independent verification**
- Optionally anchoring logs to a **blockchain (e.g., Solana via Anchor)**

---

## Core Capabilities

- **Tamper-evident logging** with hash-linked entries
- **Independent verification** of audit trails
- **Simulation and adversarial testing** (tamper scenarios)
- **Modular architecture** for distributed deployment
- **Pluggable storage** (SQLite by default)
- **Optional blockchain anchoring** for immutability guarantees

---

## System Architecture

```text
+-------------+       +-------------+       +-------------+
|   Agent     | --->  |   Storage   | --->  |  Verifier   |
| (Log Input) |       | (SQLite)    |       | (Integrity) |
+-------------+       +-------------+       +-------------+
       |                                           |
       v                                           v
+-------------+                      +-------------+
|  Simulator  |                      |   Tamper    |
| (Testing)   |                      | (Attack)    |
+-------------+                      +-------------+
       
              +----------------------+
              |   Blockchain Layer   |
              | (Solana / Anchor)    |
              +----------------------+
```

---

## Workspace Structure

This repository is organized as a **Cargo workspace**:

- **`agent`**
  Ingests logs from services and writes structured entries.
- **`verifier`**
  Validates log integrity using cryptographic proofs.
- **`simulator`**
  Generates synthetic workloads and log streams.
- **`tamper`**
  Simulates adversarial behavior by modifying logs.
- **`shared`**
  Shared libraries (hashing, schemas, utilities).
- **`dashboard`** *(optional)*
  Visualization and monitoring layer.
- **`programs`**
  Blockchain programs (e.g., Solana smart contracts via Anchor).

---

## Data Model (Simplified)

Each log entry follows a chained structure:

```rust
struct LogEntry {
    id: UUID,
    timestamp: u64,
    payload: Vec<u8>,
    prev_hash: Hash,
    current_hash: Hash,
}
```

Where:

```text
current_hash = H(payload || prev_hash || timestamp)
```

This creates a **hash chain**, making any modification detectable.

---

## Data Flow

1. **Agent** collects logs from services.
2. Logs are hashed and chained before storage.
3. Entries are stored in **SQLite**.
4. Hashes can be anchored to the blockchain (optional).
5. **Verifier** recomputes hashes to detect tampering.
6. **Tamper** module simulates attacks.

---

## Security Model

### Threats Addressed

- Log modification (post-write tampering)
- Log deletion or reordering
- Insider attacks on the storage layer

### Guarantees

- Tamper evidence via hash chaining
- Independent integrity verification
- Optional immutability via blockchain anchoring

### Limitations

- Does not prevent log deletion without external anchoring.
- Requires secure key management (if signatures are added).
- Blockchain anchoring introduces latency and cost.

---

## Prerequisites

- **Rust** (stable)
- **Cargo**
- *(Optional)* **Solana CLI** + **Anchor**

---

## Build

Run from the root directory:

```bash
cargo build --workspace
```

---

## Run Components

Execute specific modules using:

```bash
cargo run -p <package_name>
```

Examples:

```bash
cargo run -p simulator
cargo run -p agent
cargo run -p verifier
cargo run -p tamper
```

---

## Configuration

Configuration can be provided via environment variables or config files.

Typical parameters:
- **Database path** (SQLite)
- **Hashing algorithm** (default: SHA-256)
- **Blockchain RPC endpoint**
- **Anchor program ID**

---

## Testing Strategy

- **Unit tests:** hashing, chaining, validation.
- **Integration tests:** agent → storage → verifier.
- **Adversarial tests:** tamper scenarios.
- **Simulation:** load testing.

---

## Roadmap

- [ ] Merkle tree optimization for batch verification
- [ ] Digital signatures for non-repudiation
- [ ] Distributed multi-agent ingestion
- [ ] Real-time dashboard (observability)
- [ ] Full Solana/Anchor integration

---

## Use Cases

- Microservices audit logging
- Financial transaction traceability
- Compliance and audit systems
- Security-critical infrastructures

---

## License

MIT / Apache 2.0 (Please specify)
