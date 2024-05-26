use serde::{Deserialize, Serialize};

/// User account contains most of the details of a user (except password related ones).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub anonymous: bool,
    pub permissions: Vec<String>,
}

impl Default for UserAccount {
    fn default() -> Self {
        Self {
            id: 1,
            anonymous: true,
            username: "Guest".into(),
            email: "".into(),
            bio: "".into(),
            image: None,
            permissions: Vec::new(),
        }
    }
}

#[derive(Debug)]
/// It includes all user attributes that are persisted in the database.
pub struct UserEntry {
    pub user: UserAccount,
    pub password: String,
    pub salt: String,
}

// #[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
// pub struct UserPermission {
//     pub permission: String,
// }
