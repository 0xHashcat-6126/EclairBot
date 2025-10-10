use crate::impl_modlog;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct BanData {
    pub id: i64,
    pub member_id: i64,
    pub moderator_id: i64,
    pub reason: String,
    pub timestamp: i64,
}

impl_modlog!(BanData, "bans");
