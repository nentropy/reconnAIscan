-- Create the table to store general scan context information
CREATE TABLE scan_context (
    uuid TEXT PRIMARY KEY NOT NULL,         -- Unique identifier for each scan (UUID)
    timestamp INTEGER NOT NULL,             -- Timestamp of when the scan was initiated
    scan_type TEXT NOT NULL,                -- Type of scan (e.g., WiFi, IoT)
    num_clients INTEGER NOT NULL,           -- Number of clients identified during the scan
    rustscan_success BOOLEAN NOT NULL,      -- Whether Rustscan was successful
    iot_scan_success BOOLEAN NOT NULL,      -- Whether IoT scan was successful
    additional_info TEXT                    -- Any additional information relevant to the scan
);

-- Create the table to store information about identified clients
CREATE TABLE clients (
    client_id INTEGER PRIMARY KEY AUTOINCREMENT,  -- Auto-incremented client ID
    scan_uuid TEXT NOT NULL,                      -- Foreign key linking to the scan_context table
    mac_address TEXT NOT NULL,                    -- MAC address of the identified client
    ip_address TEXT NOT NULL,                     -- IP address of the identified client
    FOREIGN KEY (scan_uuid) REFERENCES scan_context (uuid) ON DELETE CASCADE
);

-- Indexes to improve lookup performance
CREATE INDEX idx_scan_uuid ON clients (scan_uuid);

-- Create the table to store the status of each scan tool
CREATE TABLE scan_tool_status (
    tool_id INTEGER PRIMARY KEY AUTOINCREMENT,    -- Auto-incremented tool status ID
    scan_uuid TEXT NOT NULL,                      -- Foreign key linking to the scan_context table
    tool_name TEXT NOT NULL,                      -- Name of the scanning tool (e.g., "Rustscan", "IoT Scan")
    success BOOLEAN NOT NULL,                     -- Whether the tool executed successfully
    error_message TEXT,                           -- Error message if the tool failed
    FOREIGN KEY (scan_uuid) REFERENCES scan_context (uuid) ON DELETE CASCADE
);

-- Indexes to improve lookup performance
CREATE INDEX idx_tool_scan_uuid ON scan_tool_status (scan_uuid);