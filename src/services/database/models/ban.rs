use sqlx::FromRow;

#[derive(FromRow)]
pub struct Ban {
    pub id: i32,
    pub member_id: i32,
    pub moderator_id: i32,
    pub reason: String,
    pub timestamp: i32,
}
