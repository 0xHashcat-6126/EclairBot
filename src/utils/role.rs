use serenity::all::RoleId;
use std::collections::HashSet;

pub fn has_any_role(member_roles: &[RoleId], allowed_roles: &[RoleId]) -> bool {
    let allowed_set: HashSet<_> = allowed_roles.iter().collect();
    member_roles.iter().any(|r| allowed_set.contains(r))
}
