extern crate std;

use std::{cell::Cell, vec};
use crate::{end::End, function::Function, reader::Reader, scan, snapshot::Snapshot, window::EcamWindow};

struct Config {
    limit: u32,
    present: u16,
    reads: Cell<usize>,
    outside: Cell<bool>,
}

impl Reader for Config {
    fn limit(&self) -> u32 { self.limit }

    fn read8(&self, requester: u16, register: u16) -> Option<u8> {
        self.reads.set(self.reads.get() + 1);
        if u32::from(requester) >= self.limit || register >= 4096 {
            self.outside.set(true);
            return None;
        }
        if requester != self.present { return Some(if register < 2 { 0xff } else { 0 }); }
        Some(match register { 0 => 0x34, 1 => 0x12, 2 => 0x78, 3 => 0x56, 11 => 1, _ => 0 })
    }
}

#[test]
fn empty_scan_page_keeps_a_work_limit_instead_of_false_eof() {
    let config = Config { limit: 300, present: 299, reads: Cell::new(0), outside: Cell::new(false) };
    let mut rows = [Function { requester: 0, vendor: 0, device: 0, class: 0 }];
    let first = scan::scan(&config, 0, &mut rows).unwrap();
    assert_eq!((first.count, first.next, first.end), (0, 256, End::WorkLimit));
    let last = scan::scan(&config, first.next, &mut rows).unwrap();
    assert_eq!((last.count, last.next, last.end), (1, 300, End::Eof));
    assert_eq!((rows[0].requester, rows[0].vendor, rows[0].device), (299, 0x1234, 0x5678));
    assert!(!config.outside.get());
    assert!(scan::scan(&config, 301, &mut rows).is_none());
}

#[test]
fn snapshot_reads_exact_bytes_and_rejects_unmapped_requesters() {
    let config = Config { limit: 1, present: 0, reads: Cell::new(0), outside: Cell::new(false) };
    let image = Snapshot::read(&config, 0).unwrap();
    assert!(image.valid());
    assert_eq!((image.vendor(), image.device(), image.class()), (0x1234, 0x5678, 0x010000));
    assert_eq!(config.reads.get(), 256);
    assert!(Snapshot::read(&config, 1).is_none());
    assert_eq!(config.reads.get(), 256);
    assert!(!config.outside.get());
}

#[test]
fn mapped_window_does_not_read_partial_function_tails() {
    let memory = vec![0xa5u8; 12288];
    let base = (memory.as_ptr() as usize + 4095) & !4095;
    // The backing allocation remains alive and covers this entire range.
    let window = unsafe { EcamWindow::new(base, 4099) }.unwrap();
    assert_eq!(window.limit(), 1);
    assert_eq!(window.read8(0, 4095), Some(0xa5));
    assert_eq!(window.read8(1, 0), None);
    assert_eq!(window.read8(0, 4096), None);
    assert!(unsafe { EcamWindow::new(base + 1, 4096) }.is_none());
    assert!(unsafe { EcamWindow::new(base, 0) }.is_none());
}
