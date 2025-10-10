#[macro_export]
macro_rules! impl_modlog {
    ($struct_name:ident, $table:expr) => {
        impl $struct_name {
            pub async fn insert(&self, pool: &sqlx::SqlitePool) -> Result<(), crate::bot::Error> {
                let query = format!(
                    "INSERT INTO {} (member_id, moderator_id, reason)
                     VALUES (?, ?, ?)
                     ON CONFLICT(id) DO NOTHING",
                    $table
                );
                sqlx::query(&query)
                    .bind(self.member_id)
                    .bind(self.moderator_id)
                    .bind(&self.reason)
                    .execute(pool)
                    .await?;
                Ok(())
            }
        }
    };
}
