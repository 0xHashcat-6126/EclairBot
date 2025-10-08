use sqlx::{FromRow, SqlitePool};
use std::error::Error;

#[derive(FromRow)]
pub struct Member {
    id: i32,
    exp: i32,
    reputation: i32,
    cash: i32,
    bank: i32,
}

impl Member {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            exp: 0,
            reputation: 0,
            cash: 0,
            bank: 0,
        }
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
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
}

#[derive(FromRow)]
pub struct Economy {
    member_id: i32,
    last_crime: i32,
    last_rob: i32,
    last_slut: i32,
    last_work: i32,
}

impl Economy {
    pub fn new(member_id: i32) -> Self {
        Self {
            member_id,
            last_crime: 0,
            last_rob: 0,
            last_slut: 0,
            last_work: 0,
        }
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO economy (member_id, last_crime, last_rob, last_slut, last_work)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(id) DO NOTHING",
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

#[derive(FromRow)]
pub struct Ban {
    id: i32,
    member_id: i32,
    moderator_id: i32,
    reason: String,
    timestamp: i32,
}

#[derive(FromRow)]
pub struct Unban {
    id: i32,
    member_id: i32,
    moderator_id: i32,
    reason: String,
    timestamp: i32,
}

#[derive(FromRow)]
pub struct Kick {
    id: i32,
    member_id: i32,
    moderator_id: i32,
    reason: String,
    timestamp: i32,
}

#[derive(FromRow)]
pub struct Warn {
    id: i32,
    member_id: i32,
    moderator_id: i32,
    reason: String,
    timestamp: i32,
}
