#[cfg(feature = "server")]
use crate::auth::Session;

use crate::server::domain::UserAccount;

#[cfg(feature = "server")]
use log::debug;

use dioxus_fullstack::prelude::*;

//pub type LoginResult = Result<UserAccount, AppError>;

#[server(Login)]
pub async fn login(email: String, password: String) -> Result<UserAccount, ServerFnError> {
    //
    let session: Session = extract().await?;
    let account = session.1.authenticate_user(email, password).await?;
    session.login_user(account.id);
    debug!("[login] Logged-in user with account: {:?}", account);
    Ok(account)
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
    let name = match session.0.current_user {
        Some(user) => user.username,
        None => "".to_string(),
    };
    Ok(name)
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    use axum_session_auth::Rights;

    let method: axum::http::Method = extract().await?;
    let session: Session = extract().await?;
    let current_user = session.current_user.clone().unwrap_or_default();

    // Let's check permissions only and not worry about if the user is anonymous or not.
    if !axum_session_auth::Auth::<UserAccount, i64, sqlx::PgPool>::build([axum::http::Method::POST], false)
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("Admin::View"),
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
