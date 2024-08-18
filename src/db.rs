/// Scanner Database
/// ----------------
/// ## Functionality:
/// 
///1. Centralized Initialization
///2. Caching
///3. Indexing on key writes for realtime updates

use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::{Connection, Result};

struct ScanContext {
    uuid: String,
    timestamp: u64,
    db_conn: Connection,
}

impl ScanContext {
    fn new(db_path: &str) -> Result<Self> {
        let uuid = Uuid::new_v4().to_string();
        let start_time = SystemTime::now();
        let timestamp = start_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let db_conn = Connection::open(db_path)?;

        Ok(Self {
            uuid,
            timestamp,
            db_conn,
        })
    }
}

fn main() -> Result<()> {
    let scan_context = ScanContext::new("scan_results.db")?;

    // Pass the context to each scan type
    scan_type_1(&scan_context)?;
    scan_type_2(&scan_context)?;
    // Other scan types...

    Ok(())
}

fn insert_scan_results(context: &ScanContext, results: Vec<Chipset>) -> Result<()> {
    let tx = context.db_conn.transaction()?;
    
    for chipset in results {
        tx.execute(
            "INSERT INTO chipsets (id, name, popularity_score, device_id) VALUES (?1, ?2, ?3, ?4)",
            &[&chipset.id, &chipset.name, &chipset.popularity_score, &chipset.device_id],
        )?;
    }

    tx.commit()?;
    Ok(())
}
let mut stmt = context.db_conn.prepare("SELECT * FROM devices WHERE mac_address = ?1")?;
let mut rows = stmt.query(&[mac_address])?;


//preserving data consistency.

fn insert_scan(context: &ScanContext, scan: Scan) -> Result<()> {
    let tx = context.db_conn.transaction()?;
    tx.execute(
        "INSERT INTO scans (id, timestamp, scan_type) VALUES (?1, ?2, ?3)",
        &[&scan.id, &scan.timestamp, &scan.scan_type],
    )?;
    
    // Additional operations...

    tx.commit()?;
    Ok(())
}
fn main() -> Result<()> {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run() -> Result<()> {
    let scan_context = ScanContext::new("scan_results.db")?;
    // Application logic here...
    Ok(())
}
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::{Connection, Result};

struct ScanContext {
    uuid: String,
    timestamp: u64,
    db_conn: Connection,
}

impl ScanContext {
    fn new(db_path: &str) -> Result<Self> {
        let uuid = Uuid::new_v4().to_string();
        let start_time = SystemTime::now();
        let timestamp = start_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let db_conn = Connection::open(db_path)?;

        Ok(Self {
            uuid,
            timestamp,
            db_conn,
        })
    }
}

fn main() -> Result<()> {

    let scan_context = ScanContext::new("scan_results.db")?;

    // Example of passing the context to a specific scan type
    example_1(&scan_context)?;
    example_2(&scan_context)?;
    // Other scan types...

    Ok(())
}

fn example_scan(context: &ScanContext) -> Result<()> {
    // Example scan logic
    let results = vec![]; // Placeholder for actual scan results
    insert_scan_results(context, results)?;
    Ok(())
}
