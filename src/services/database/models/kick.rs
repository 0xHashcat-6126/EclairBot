use crate::impl_modlog;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct KickData {
    pub id: i64,
    pub member_id: i64,
    pub moderator_id: i64,
    pub reason: String,
    pub timestamp: i64,
}

pub fn new(member_id: i64, moderator_id: i64, reason: String) -> KickData {
    KickData {
        id: 0,
        member_id,
        moderator_id,
        reason,
        timestamp: 0,
    }
}

impl_modlog!(KickData, "kicks");
