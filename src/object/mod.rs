//! Objects within the `YubiHSM 2` (keys, certificates, or other opaque data)
//!
//! For more information, see:
//! <https://developers.yubico.com/YubiHSM2/Concepts/Object.html>

pub(crate) mod commands;
mod entry;
mod filter;
pub(crate) mod generate;
mod handle;
pub mod import;
mod info;
mod label;
mod origins;
mod types;

pub use self::{
    entry::Entry,
    filter::Filter,
    handle::Handle,
    info::Info,
    label::{Label, LABEL_SIZE},
    origins::Origin,
    types::Type,
};

/// Object identifiers
pub type Id = u16;

/// Sequence identifiers: number of times an object with a given ID has been
/// created in this `YubiHSM` (with all previous versions having been deleted)
pub type SequenceId = u8;
