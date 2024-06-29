use dioxus::prelude::Signal;
use dioxus_sdk::storage::*;
use serde::{Deserialize, Serialize};

use crate::server::UserAccount;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub current_user: Option<UserAccount>,
}

impl State {
    /// LocalStorage key.
    const LS_KEY: &'static str = "tmc";

    pub fn load_from_localstorage() -> Signal<Self> {
        let state_sgnl = use_synced_storage::<LocalStorage, State>(Self::LS_KEY.into(), || State::default());
        log::debug!(">>> [load_from_localstorage] Loaded {:?}", state_sgnl());
        state_sgnl
    }

    pub fn save_to_localstorage(&self) {
        LocalStorage::set(Self::LS_KEY.into(), self);
        log::debug!(">>> [save_to_localstorage] Saved {:?}", self);
    }

    pub fn new(user: &UserAccount) -> Self {
        Self {
            current_user: Some(user.clone()),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
