use dioxus::prelude::*;

#[cfg(feature = "server")]
use log::debug;

#[server(SaveUserProfilePrimaryInfo)]
pub async fn save_user_profile_primary_info(username: String, email: String, bio: String) -> Result<(), ServerFnError> {
    //
    debug!(
        "[save_user_profile_primary_info] Received: username: {}, email: {}, bio: {}",
        username, email, bio
    );

    Ok(())
}
