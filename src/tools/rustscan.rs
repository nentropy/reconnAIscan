/// # Using Rustscan
/// ## Port Scanning Functionality
/// ------------------------------------------------------
/// ## Functionality:
/// Nmap scanning but with times as low as 3 seconds for 2000 ports
/// 

use uuid::uuidv4;   
use rustscan::scanner::Scanner;
use rustscan::port_strategy::PortStrategy;
use std::sync::Arc;
use std::time::Duration;

enum TargetList {
    List(Vec<String>),
    Range(String, String),
    Single(String),
}

struct ScanInstance {
    uuid: String,
    scan_type: Options<String>,
    port_strategy: Options<PortStrategy>,
    target_list: Options<TargetList>,
}

impl ScanInstance {
    pub fn new(scan_type: Option<String>, port_strategy: Option<String>, target_list: Option<String>) -> Self  {
        ScanInstance {
            uuid: Uuid::new_v4().to_string(),
            scan_type,
            port_strategy,
            target_list,
        }
    }

    pub async fn init_rustscan(&self) {
        // Create a new Scanner instance
        let mut scan = Scanner::new();
        
        // Set the port strategy if provided
        if let Some(strategy) = &self.port_strategy {
            scan = scan.port_strategy(strategy.clone());
        }

        // Set the target list if provided
        if let Some(targets) = &self.target_list {
            match targets {
                TargetList::List(list) => {
                    for target in list {
                        scan = scan.add_target(target.to_string());
                    }
                }
                TargetList::Range(start, end) => {
                    // Assuming a function to generate the range of IPs
                    let range = generate_ip_range(start, end);
                    for target in range {
                        scan = scan.add_target(target);
                    }
                }
                TargetList::Single(target) => {
                    scan = scan.add_target(target.to_string());
                }
            }
        }

        // Execute the scan asynchronously
        let result = scan.run().await;

        // Handle the result
        match result {
            Ok(_) => println!("Scan completed successfully"),
            Err(e) => eprintln!("Scan failed: {:?}", e),
        }
    }
}

    fn generate_ip_range(start: &str, end: &str) -> Vec<String> {
        // Implement logic to generate a range of IP addresses
        vec![start.to_string(), end.to_string()]
    }

    #[tokio::main]
    async fn main() {
        // Example usage
        let scan_instance = ScanInstance::new(
            Some("TCP Scan".to_string()),
            Some(PortStrategy::default()),
            Some(TargetList::Single("127.0.0.1".to_string())),
        );

        // Running the scan in a separate task for concurrency
        let scan_task = task::spawn(async move {
            scan_instance.init_rustscan().await;
        });

        // Wait for the scan to complete
        scan_task.await.unwrap();
    }
        



