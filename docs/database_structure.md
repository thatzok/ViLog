### Database Structure for ViLog

ViLog supports writing DTC (Diagnostic Trouble Code) entries to MySQL, PostgreSQL, and SQLite. The data structure is consistent across all three database systems.

#### Table: `logs`

This table stores the diagnostic entries received from the heat pump.

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | INTEGER/SERIAL | Primary Key (auto-increment) |
| `timestamp` | BIGINT | Unix timestamp in milliseconds |
| `iso_date` | VARCHAR(32) | ISO 8601 formatted date string |
| `systemid` | VARCHAR(64) | ID of the heat pump system |
| `ecuid` | VARCHAR(64) | ID of the ECU (Electronic Control Unit) |
| `state_id` | INTEGER | Internal state ID from the device |
| `state_type` | VARCHAR(32) | Type of state (e.g., Status, Info, Warning, Error) |
| `severity` | VARCHAR(32) | Severity level |
| `code` | VARCHAR(32) | Diagnostic message code (e.g., F.123) |
| `message` | TEXT | Description text of the diagnostic entry |

---

#### SQL Creation Scripts

##### SQLite

```sql
CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp BIGINT NOT NULL,
    iso_date TEXT NOT NULL,
    systemid TEXT NOT NULL,
    ecuid TEXT NOT NULL,
    state_id INTEGER NOT NULL,
    state_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    code TEXT NOT NULL,
    message TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_logs_systemid ON logs(systemid);
```

##### MySQL / MariaDB

```sql
CREATE TABLE IF NOT EXISTS logs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    timestamp BIGINT NOT NULL,
    iso_date VARCHAR(32) NOT NULL,
    systemid VARCHAR(64) NOT NULL,
    ecuid VARCHAR(64) NOT NULL,
    state_id INT NOT NULL,
    state_type VARCHAR(32) NOT NULL,
    severity VARCHAR(32) NOT NULL,
    code VARCHAR(32) NOT NULL,
    message TEXT NOT NULL,
    INDEX (timestamp),
    INDEX (systemid)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

##### PostgreSQL

```sql
CREATE TABLE IF NOT EXISTS logs (
    id SERIAL PRIMARY KEY,
    timestamp BIGINT NOT NULL,
    iso_date VARCHAR(32) NOT NULL,
    systemid VARCHAR(64) NOT NULL,
    ecuid VARCHAR(64) NOT NULL,
    state_id INTEGER NOT NULL,
    state_type VARCHAR(32) NOT NULL,
    severity VARCHAR(32) NOT NULL,
    code VARCHAR(32) NOT NULL,
    message TEXT NOT NULL
);

CREATE INDEX idx_logs_timestamp ON logs(timestamp);
CREATE INDEX idx_logs_systemid ON logs(systemid);
```
