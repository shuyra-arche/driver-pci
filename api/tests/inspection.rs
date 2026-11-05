use env::PieToken;
use pci_api::inspection::{Call, Page, Scan, BACK, CAPACITY, EOF};
use wire::Message;

#[test]
fn inspection_request_keeps_encoded_length_and_bounds() {
    let request = Scan { from: 65536, back: PieToken::mint(17) };
    let mut bytes = [0xa5; 16];
    let len = request.store(&mut bytes).unwrap();
    assert_eq!(len, 12);
    assert_eq!(Scan::fetch(&bytes[..len]), Some(request));
    assert_eq!(Call::back(&request), request.back);
    assert_eq!(Call::BACK, BACK);
    assert!(Scan::fetch(&bytes[..len - 1]).is_none());
    assert!(Scan::fetch(&bytes[..len + 1]).is_none());
    assert!(Scan { from: 65537, ..request }.store(&mut bytes).is_none());
    bytes[..4].copy_from_slice(&65537u32.to_le_bytes());
    assert!(Scan::fetch(&bytes[..len]).is_none());
}

#[test]
fn inspection_reply_rejects_inconsistent_snapshot_geometry() {
    let mut reply = Page::of(wire::OK);
    reply.next = 256;
    reply.requester = 255;
    reply.count = 1;
    reply.end = CAPACITY;
    reply.bytes[0..2].copy_from_slice(&0x1234u16.to_le_bytes());
    let mut bytes = [0; Page::LEN];
    assert_eq!(reply.store(&mut bytes), Some(265));
    assert_eq!(Page::fetch(&bytes), Some(reply));
    assert!(Page::fetch(&bytes[..264]).is_none());
    let mut extra = bytes.to_vec();
    extra.push(0);
    assert!(Page::fetch(&extra).is_none());
    bytes[7] = 2;
    assert!(Page::fetch(&bytes).is_none());
    reply.next = u32::from(reply.requester);
    assert!(reply.store(&mut bytes).is_none());
    reply.next = 256;
    reply.count = 0;
    assert!(reply.store(&mut bytes).is_none());
    reply.end = EOF;
    assert_eq!(reply.store(&mut bytes), Some(Page::LEN));
}
