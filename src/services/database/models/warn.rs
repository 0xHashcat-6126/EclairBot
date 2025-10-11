use crate::impl_modlog;
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Debug)]
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
    /// Pobiera ostrzeżenia dla danego użytkownika
    pub async fn get_by_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, WarnData>(
            "SELECT id, member_id, moderator_id, reason, timestamp FROM warns WHERE member_id = ? ORDER BY timestamp DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Upewnia się, że użytkownik istnieje w tabeli members
    pub async fn ensure_member_exists(pool: &SqlitePool, user_id: i64) -> Result<(), sqlx::Error> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM members WHERE id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        if exists == 0 {
            sqlx::query("INSERT INTO members (id) VALUES (?)")
                .bind(user_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}

impl_modlog!(WarnData, "warns");
