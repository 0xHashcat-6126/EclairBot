use sqlx::{Connection, SqliteConnection};
use std::error::Error;

const DATABASE_URL: &str = "sqlite://bot.db";

async fn init() -> Result<(), Box<dyn Error>> {
    let mut conn = SqliteConnection::connect(DATABASE_URL).await?;

    let tables = [
        "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            exp INTEGER NOT NULL DEFAULT 0,
            cash INTEGER NOT NULL DEFAULT 0,
            bank INTEGER NOT NULL DEFAULT 0
        )",
        "
        CREATE TABLE IF NOT EXISTS economy (
            user_id TEXT PRIMARY KEY,
            last_crime INTEGER NOT NULL DEFAULT 0,
            last_rob INTEGER NOT NULL DEFAULT 0,
            last_slut INTEGER NOT NULL DEFAULT 0,
            last_work INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        "
        CREATE TABLE IF NOT EXISTS warns (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            moderator_id TEXT NOT NULL,
            reason TEXT NOT NULL,
            timestamp INTEGER NOT NULL DEFAULT (strftime('%s','now')),
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY(moderator_id) REFERENCES users(id) ON DELETE CASCADE
        )",
    ];

    let mut tx = conn.begin().await?;
    for table_sql in &tables {
        sqlx::query(table_sql).execute(tx.as_mut()).await?;
    }
    tx.commit().await?;

    Ok(())
}
