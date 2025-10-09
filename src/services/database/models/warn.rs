use sqlx::FromRow;

#[derive(FromRow)]
pub struct Warn {
    id: i32,
    member_id: i32,
    moderator_id: i32,
    reason: String,
    timestamp: i32,
}
