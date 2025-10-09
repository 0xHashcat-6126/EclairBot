use crate::bot::Error;
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow)]
pub struct Member {
    pub id: i64,
    pub exp: u32,
    pub reputation: i32,
    pub cash: i32,
    pub bank: i32,
}

pub fn new(id: i64) -> Member {
    Member {
        id,
        exp: 0,
        reputation: 0,
        cash: 0,
        bank: 0,
    }
}

pub async fn get_member(pool: &SqlitePool, id: i64) -> Result<Member, Error> {
    let member = sqlx::query_as::<_, Member>("SELECT * FROM members WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(member)
}

impl Member {
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO members (id, exp, reputation, cash, bank)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(id) DO NOTHING",
        )
        .bind(self.id)
        .bind(self.exp)
        .bind(self.reputation)
        .bind(self.cash)
        .bind(self.bank)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn add_exp(&mut self, pool: &SqlitePool, exp: u32) -> Result<(), Error> {
        self.exp += exp;

        sqlx::query("UPDATE members SET exp = ? WHERE id = ?")
            .bind(self.exp)
            .bind(self.id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
