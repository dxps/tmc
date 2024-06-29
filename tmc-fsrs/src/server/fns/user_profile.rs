use dioxus_fullstack::prelude::*;

#[cfg(feature = "server")]
use log::debug;

#[cfg(feature = "server")]
use crate::{auth::Session, server::UserAccount};

#[server(SaveUserProfilePrimaryInfo)]
pub async fn save_user_profile_primary_info(
    id: String,
    username: String,
    email: String,
    bio: String,
) -> Result<(), ServerFnError> {
    //
    debug!(
        "[save_user_profile_primary_info] Received: id: {}, username: {}, email: {}, bio: {}",
        id, username, email, bio
    );

    let session: Session = extract().await?;
    let mut ua = UserAccount::default();
    ua.id = id;
    ua.username = username;
    ua.email = email;
    ua.bio = bio;
    session
        .1
        .update_user_account(ua)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()))
}
