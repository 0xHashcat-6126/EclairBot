use sqlx::FromRow;

#[derive(FromRow)]
pub struct Member {
    id: i32,
    exp: i32,
    reputation: i32,
    cash: i32,
    bank: i32,
}

#[derive(FromRow)]
pub struct Economy {
    member_id: i32,
    last_crime: i32,
    last_rob: i32,
    last_slut: i32,
    last_work: i32,
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
