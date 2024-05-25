#[cfg(feature = "server")]
use crate::auth::{Session, User};
use crate::server::{model::User, AppError};
#[cfg(feature = "server")]
use log::debug;

use dioxus_fullstack::prelude::*;

pub type LoginResult = Result<User, AppError>;

#[server(Login)]
pub async fn login(email: String, password: String) -> Result<User, ServerFnError> {
    //
    debug!("[login] Received email:'{}', password:'{}'", email, password);
    let session: Session = extract().await?;
    let user_entry = session.1.get_by_email(&email, crate::server::AppUseCase::UserLogin).await;
    debug!("[login] user_entry: {:?}", user_entry);
    session.login_user(2);
    Ok(())
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    session.logout_user();
    Ok(())
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    Ok(session.0.current_user.unwrap().username.to_string())
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    let method: axum::http::Method = extract().await?;
    let session: Session = extract().await?;
    let current_user = session.current_user.clone().unwrap_or_default();

    // Lets check permissions only and not worry about if the user is anonymous or not.
    if !axum_session_auth::Auth::<User, i64, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(axum_session_auth::Rights::any([
            axum_session_auth::Rights::permission("Category::View"),
            axum_session_auth::Rights::permission("Admin::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
    {
        return Ok(format!(
            "User '{}' does not have permissions needed to view this page. Please login.",
            current_user.username
        ));
    }

    Ok(format!(
        "User '{}' has the needed permissions to view this page. Here are his permissions: {:?}",
        current_user.username, current_user.permissions
    ))
}
