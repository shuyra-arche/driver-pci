#![no_std]
#![no_main]

use execution::boot::Report;

#[mold::entry]
fn main() -> Report<'static> {
    match pci_server::serve() {
        Ok(()) => Report::note(env::EXIT_OK, "pci: done"),
        Err(why) => Report::note(env::EXIT_FAULT, why),
    }
}
