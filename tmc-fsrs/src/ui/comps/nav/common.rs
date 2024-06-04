use dioxus::prelude::*;

/// Navigation header specific function to highlight the link based on the current path.
pub fn style_link(curr_path: &String, link_path: String) -> &'static str {
    if *curr_path == link_path {
        "text-sm text-green-600 py-2 px-4 hover:bg-gray-50 rounded-lg transition duration-200"
    } else {
        "text-sm text-gray-600 py-2 px-4 hover:bg-gray-50 rounded-lg transition duration-200"
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct NavProps {
    #[props(default = "home".to_string())]
    pub active_path: String,
}

impl NavProps {
    pub fn home() -> String {
        "home".to_string()
    }
    pub fn blog() -> String {
        "blog".to_string()
    }
    pub fn sample() -> String {
        "sample".to_string()
    }
    pub fn login() -> String {
        "login".to_string()
    }
}