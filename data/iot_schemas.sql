CREATE TABLE IF NOT EXISTS scans (
    id TEXT PRIMARY KEY,
    timestamp INTEGER,
    scan_type TEXT
);

CREATE TABLE IF NOT EXISTS chipsets (
    id TEXT PRIMARY KEY,
    name TEXT,
    device_id TEXT,
    popularity_score REAL,
    security_score BOOLEAN,
);

CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY,
    chipset_id TEXT,
    mac_address TEXT,
    FOREIGN KEY(chipset_id) REFERENCES chipsets(id)
);

CREATE TABLE IF NOT EXISTS device_popularity (
    id TEXT,
    chipset_id TEXT PRIMARY KEY,
    mac_address TEXT,
    TIME_ALIVE INTEGER,
    FOREIGN KEY(chipset_id) REFERENCES chipsets(id),
);


