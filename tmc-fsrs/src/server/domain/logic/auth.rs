use std::sync::Arc;

use crate::server::{AppError, AppUseCase, UserAccount, UsersRepo};

#[derive(Clone)]
pub struct AuthMgr {
    user_repo: Arc<UsersRepo>,
}

impl AuthMgr {
    //
    pub fn new(user_repo: Arc<UsersRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(&self, email: String, username: String, pwd: String) -> Result<String, AppError> {
        //
        let (pwd, salt) = Self::generate_password(pwd);
        self.user_repo.save(email, username, pwd, salt).await
    }

    pub async fn authenticate_user(&self, email: String, pwd: String) -> Result<UserAccount, AppError> {
        //
        let user_entry = self.user_repo.get_by_email(&email, AppUseCase::UserLogin).await?;
        match Self::check_password(&pwd, &user_entry.password, &user_entry.salt) {
            true => Ok(user_entry.into()),
            false => Err(AppError::Unauthorized("wrong credentials".into())),
        }
    }

    fn generate_password(pwd: String) -> (String, String) {
        //
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric).take(12).collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }

    fn check_password(input_pwd: &str, pwd: &str, salt: &str) -> bool {
        //
        let digest = md5::compute(format!("@{salt}${input_pwd}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}
