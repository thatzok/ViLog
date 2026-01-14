use sqlx::{MySqlPool, PgPool, SqlitePool, ConnectOptions};
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;
use crate::config::SqlResolved;
use crate::dtc::ListEntryDtc;
use log::{error, info};
use std::str::FromStr;

pub struct DatabasePools {
    pub mysql: Option<MySqlPool>,
    pub postgres: Option<PgPool>,
    pub sqlite: Option<SqlitePool>,
}

impl DatabasePools {
    pub async fn init(
        mysql_cfg: &SqlResolved,
        postgres_cfg: &SqlResolved,
        sqlite_cfg: &SqlResolved,
    ) -> Self {
        let mysql = if mysql_cfg.enabled {
            match MySqlConnectOptions::from_str(&mysql_cfg.url) {
                Ok(opts) => {
                    let opts = opts.disable_statement_logging();
                    match MySqlPool::connect_with(opts).await {
                        Ok(pool) => {
                            info!("Connected to MySQL at {}", mysql_cfg.url);
                            Some(pool)
                        }
                        Err(e) => {
                            error!("Failed to connect to MySQL: {}", e);
                            None
                        }
                    }
                },
                Err(e) => {
                    error!("Invalid MySQL URL: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let postgres = if postgres_cfg.enabled {
            match PgConnectOptions::from_str(&postgres_cfg.url) {
                Ok(opts) => {
                    let opts = opts.disable_statement_logging();
                    match PgPool::connect_with(opts).await {
                        Ok(pool) => {
                            info!("Connected to PostgreSQL at {}", postgres_cfg.url);
                            Some(pool)
                        }
                        Err(e) => {
                            error!("Failed to connect to PostgreSQL: {}", e);
                            None
                        }
                    }
                },
                Err(e) => {
                    error!("Invalid PostgreSQL URL: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let sqlite = if sqlite_cfg.enabled {
            match SqliteConnectOptions::from_str(&sqlite_cfg.url) {
                Ok(opts) => {
                    let opts = opts.disable_statement_logging().create_if_missing(true);
                    match SqlitePool::connect_with(opts).await {
                        Ok(pool) => {
                            info!("Connected to SQLite at {}", sqlite_cfg.url);
                            Some(pool)
                        }
                        Err(e) => {
                            error!("Failed to connect to SQLite: {}", e);
                            None
                        }
                    }
                },
                Err(e) => {
                    error!("Invalid SQLite URL: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Self {
            mysql,
            postgres,
            sqlite,
        }
    }

    pub async fn write_entries(
        &self,
        entries: Vec<ListEntryDtc>,
        systemid: String,
        ecuid: String,
    ) {
        for entry in entries {
            let timestamp = entry.date_time.timestamp;
            let iso_date = entry.get_iso8601_from_timestamp();
            let state_id = entry.state.id;
            let state_type = entry.state_type.clone();
            let severity = entry.get_severity();
            let code = entry.get_msg_code();
            let message = entry.state.text.clone();

            if let Some(ref pool) = self.mysql {
                let pool = pool.clone();
                let sid = systemid.clone();
                let eid = ecuid.clone();
                let s_type = state_type.clone();
                let sev = severity.clone();
                let c = code.clone();
                let msg = message.clone();
                let iso = iso_date.clone();

                tokio::spawn(async move {
                    if let Err(e) = sqlx::query(
                        "INSERT INTO logs (timestamp, iso_date, systemid, ecuid, state_id, state_type, severity, code, message) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(timestamp)
                    .bind(iso)
                    .bind(sid)
                    .bind(eid)
                    .bind(state_id)
                    .bind(s_type)
                    .bind(sev)
                    .bind(c)
                    .bind(msg)
                    .execute(&pool)
                    .await
                    {
                        error!("MySQL write error: {}", e);
                    }
                });
            }

            if let Some(ref pool) = self.postgres {
                let pool = pool.clone();
                let sid = systemid.clone();
                let eid = ecuid.clone();
                let s_type = state_type.clone();
                let sev = severity.clone();
                let c = code.clone();
                let msg = message.clone();
                let iso = iso_date.clone();

                tokio::spawn(async move {
                    if let Err(e) = sqlx::query(
                        "INSERT INTO logs (timestamp, iso_date, systemid, ecuid, state_id, state_type, severity, code, message) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
                    )
                    .bind(timestamp)
                    .bind(iso)
                    .bind(sid)
                    .bind(eid)
                    .bind(state_id)
                    .bind(s_type)
                    .bind(sev)
                    .bind(c)
                    .bind(msg)
                    .execute(&pool)
                    .await
                    {
                        error!("PostgreSQL write error: {}", e);
                    }
                });
            }

            if let Some(ref pool) = self.sqlite {
                let pool = pool.clone();
                let sid = systemid.clone();
                let eid = ecuid.clone();
                let s_type = state_type.clone();
                let sev = severity.clone();
                let c = code.clone();
                let msg = message.clone();
                let iso = iso_date.clone();

                tokio::spawn(async move {
                    if let Err(e) = sqlx::query(
                        "INSERT INTO logs (timestamp, iso_date, systemid, ecuid, state_id, state_type, severity, code, message) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(timestamp)
                    .bind(iso)
                    .bind(sid)
                    .bind(eid)
                    .bind(state_id)
                    .bind(s_type)
                    .bind(sev)
                    .bind(c)
                    .bind(msg)
                    .execute(&pool)
                    .await
                    {
                        error!("SQLite write error: {}", e);
                    }
                });
            }
        }
    }
}
