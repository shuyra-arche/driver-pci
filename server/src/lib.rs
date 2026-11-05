#![no_std]

//! PCI-owned ECAM enumeration, exact BAR grants and authenticated resource effects.

extern crate alloc;

use pci_api as api;
use pci_declaration::bindings;

mod bar;
mod core;
mod ecam;
mod end;
mod endpoint;
mod function;
mod function_state;
mod inspection;
mod lookup;
mod page;
mod placement;
mod place_request;
mod platform;
mod reader;
mod scan;
mod serve;
mod snapshot;
mod window;

pub use serve::serve;

#[cfg(test)]
mod tests;
