use sqlx::{FromRow, SqlitePool};
use crate::bot::Error;

#[derive(FromRow)]
pub struct WarnData {
    pub id: i64,
    pub member_id: i64,
    pub moderator_id: i64,
    pub reason: String,
    pub timestamp: i64,
}

pub fn new(member_id: i64, moderator_id: i64, reason: String) -> WarnData {
    WarnData {
        id: 0,
        member_id,
        moderator_id,
        reason,
        timestamp: 0,
    }
}

impl WarnData {
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO warns (member_id, moderator_id, reason)
            VALUES (?, ?, ?)
            ON CONFLICT(id) DO NOTHING",
        )
            .bind(self.member_id)
            .bind(self.moderator_id)
            .bind(&self.reason)
            .execute(pool)
            .await?;

        Ok(())
    }
}