#[cfg(feature = "server")]
mod database;
#[cfg(feature = "server")]
pub use database::*;

pub mod fns;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;
