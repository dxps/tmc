#[cfg(feature = "server")]
mod database;

pub mod fns;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
pub use {database::*, server::*};
