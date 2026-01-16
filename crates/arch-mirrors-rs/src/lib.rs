//! # Arch Mirrors
//! Get and parse the Arch Linux mirrors.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::invalid_codeblock_attributes)]
pub mod mirror;
pub mod protocol;
pub mod status;

pub use crate::mirror::Mirror;
pub use protocol::Protocol;
pub use status::Status;
