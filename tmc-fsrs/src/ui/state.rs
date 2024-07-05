use crate::server::UserAccount;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub current_user: Option<UserAccount>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    localstorage: Option<web_sys::Storage>,
}

impl State {
    /// LocalStorage key.
    const LS_KEY: &'static str = "tmc";

    fn new() -> Result<Self, String> {
        let window = web_sys::window().expect("no global `window` exists");
        if let Ok(Some(storage)) = window.local_storage() {
            let state = State {
                current_user: None,
                localstorage: Some(storage),
            };
            Ok(state)
        } else {
            debug!(">>> [State::new] Error: No local storage found!");
            Err("No local storage found".into())
        }
    }

    pub fn load_from_localstorage() -> Result<Self, String> {
        let mut state = State::new()?;
        if let Ok(Some(value)) = state.localstorage.as_ref().unwrap().get(Self::LS_KEY) {
            debug!(">>> [State::load_from_localstorage] Loaded value={:?}", value);
            state.current_user = Some(serde_json::from_str(&value).unwrap());
        } else {
            debug!(">>> [State::load_from_localstorage] No value exists in localstorage.");
        }
        Ok(state)
    }

    pub fn save_to_localstorage(&self) {
        //
        if self.current_user.is_some() {
            self.localstorage
                .as_ref()
                .unwrap()
                .set_item(Self::LS_KEY, &serde_json::to_string(&self.current_user).unwrap())
                .unwrap();
            debug!(">>> [save_to_localstorage] Saved {:?}", self.current_user);
        } else {
            self.localstorage.as_ref().unwrap().remove_item(Self::LS_KEY).unwrap();
            debug!(">>> [save_to_localstorage] Removed {:?} key from localstorage.", Self::LS_KEY);
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
