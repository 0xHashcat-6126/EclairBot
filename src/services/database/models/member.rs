use crate::bot::Error;
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow)]
pub struct MemberData {
    pub id: i64,
    pub exp: i64,
    pub reputation: i64,
    pub cash: i64,
    pub bank: i64,
}

impl MemberData {
    /// Tworzy nową instancję MemberData z domyślnymi wartościami
    pub fn new(id: i64) -> Self {
        Self {
            id,
            exp: 0,
            reputation: 0,
            cash: 0,
            bank: 0,
        }
    }

    /// Wstawia użytkownika do bazy danych, jeśli go jeszcze nie ma
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

    /// Dodaje exp do użytkownika
    pub async fn add_exp(&mut self, pool: &SqlitePool, exp: i64) -> Result<(), Error> {
        self.exp += exp;

        sqlx::query("UPDATE members SET exp = ? WHERE id = ?")
            .bind(self.exp)
            .bind(self.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Upewnia się, że użytkownik istnieje w tabeli members
    pub async fn ensure_exists(pool: &SqlitePool, id: i64) -> Result<(), Error> {
        let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM members WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        if exists == 0 {
            let member = MemberData::new(id);
            member.insert(pool).await?;
        }

        Ok(())
    }
}

/// Pobiera użytkownika z bazy danych
pub async fn get_member(pool: &SqlitePool, id: i64) -> Result<MemberData, Error> {
    let member = sqlx::query_as::<_, MemberData>("SELECT * FROM members WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(member)
}
