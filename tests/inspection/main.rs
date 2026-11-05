#![no_std]
#![no_main]

use env::Wait;
use ipc::{rpc::request::Sender, time::Deadline};
use pci_api::inspection::{Call, ENTRY, ENTRY_PATH, EOF, Scan};

const WAIT: Wait = Wait::AtMost(5000);

#[mold::entry]
fn main() -> Result<(), env::Reason> {
    let session =
        ipc::session::Session::open(env::unit::sire(), system_client::operator::BERTH, WAIT)
            .map_err(|_| 1usize)?;
    let tree = system_client::operator::Face::of(session);
    let service = tree
        .resolve(system_api::namespace::SERVICE, WAIT)
        .map_err(|_| 2usize)?;
    let path = system_api::operator::Path::new(ENTRY_PATH);
    let entry = tree
        .tile_from(service, path, WAIT)
        .and_then(|tile| tile.token(WAIT))
        .map_err(|_| 3usize)?;
    let result = inspect(entry);
    env::pie::release(entry, env::ReleaseMode::Revoke).map_err(|_| 10usize)?;
    result
}

fn inspect(entry: env::PieToken) -> Result<(), env::Reason> {
    let (_, _, mark) = resource::raw::reserve(entry).map_err(|_| 4usize)?;
    if mark != ENTRY {
        return Err(5);
    }
    let sender = Sender::<Call>::from_raw(entry, Call::BACK).map_err(|_| 6usize)?;
    let first = sender
        .call(Deadline::new(WAIT), |back| Scan { from: 0, back })
        .map_err(|_| 7usize)?;
    if first.status != 0
        || first.count != 1
        || first.next == 0
        || matches!(
            u16::from_le_bytes([first.bytes[0], first.bytes[1]]),
            0 | 0xffff
        )
    {
        return Err(8);
    }
    let end = sender
        .call(Deadline::new(WAIT), |back| Scan { from: 65536, back })
        .map_err(|_| 9usize)?;
    if end.status != 0 || end.count != 0 || end.end != EOF || end.next != 65536 {
        return Err(11);
    }
    Ok(())
}
