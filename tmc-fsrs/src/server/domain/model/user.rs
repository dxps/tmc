use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::Attribute;

/// User account contains most of the details of a user (except password related ones).
#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: String,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub is_anonymous: bool,
    pub permissions: Vec<String>,
    pub attributes: Vec<Attribute>,
}

#[cfg(feature = "server")]
impl Default for UserAccount {
    fn default() -> Self {
        Self {
            id: super::create_id(),
            is_anonymous: true,
            username: "Guest".into(),
            email: "".into(),
            bio: "".into(),
            image: None,
            permissions: Vec::new(),
            attributes: Vec::new(),
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

impl From<UserEntry> for UserAccount {
    fn from(entry: UserEntry) -> Self {
        entry.user
    }
}

/// It includes just the user's password and salt.
pub struct UserPasswordSalt {
    pub password: String,
    pub salt: String,
}
