use crate::impl_modlog;
use sqlx::{FromRow, SqlitePool};

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
    pub async fn get_by_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, WarnData>(
            "SELECT id, member_id, moderator_id, reason, timestamp FROM warns WHERE member_id = ? ORDER BY timestamp DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

}

impl_modlog!(WarnData, "warns");
