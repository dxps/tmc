/// The main representation of the User. <br/>
/// It contains most of the details (except for password).
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

#[derive(Debug)]
/// It includes all user attributes that are persisted in the database.
pub struct UserEntry {
    pub user: User,
    pub password: String,
    pub salt: String,
}
