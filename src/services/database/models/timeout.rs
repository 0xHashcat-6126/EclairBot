use crate::bot::Error;
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow)]
pub struct Timeouts {
    member_id: i64,
    last_crime: i32,
    last_rob: i32,
    last_slut: i32,
    last_work: i32,
}

pub fn new(member_id: i64) -> Timeouts {
    Timeouts {
        member_id,
        last_crime: 0,
        last_rob: 0,
        last_slut: 0,
        last_work: 0,
    }
}

impl Timeouts {
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO timeouts (member_id, last_crime, last_rob, last_slut, last_work)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(member_id) DO NOTHING",
        )
        .bind(self.member_id)
        .bind(self.last_crime)
        .bind(self.last_rob)
        .bind(self.last_slut)
        .bind(self.last_work)
        .execute(pool)
        .await?;

        Ok(())
    }
}
