# PCI Unit intake validation

The candidate continues runtime checkpoint
`bc0da12a1bafbf684179301853b5709a1ddfeaf9`. Production Rust, public protocols,
standalone dependency manifests and root Cargo.lock are unchanged. The follow-up
adds dynamic Unit policy, three host declaration checks and one new native
read-only inspection probe. No legacy suite or scene is migrated.

Validation used nightly-2026-10-09. The retained standalone RV64 release build,
five API cases, three server cases and three new Unit contract cases passed
with locked/offline dependencies. Scoped formatting and `git diff --check`
passed. Existing runtime dead-code warnings remain.

Vanllia main `c61cd03` was checked out with its exact recursive provider pins.
PCI was added as an explicit local source in an isolated checkout; no shared
composition branch was changed. System already owns the trusted ECAM/aperture
supplies, and its metadata already lists those generated resources. The current
Env/Wire/Mold/Unit and System/Driver overlays resolve one selected identity of
each SDK package without rewriting historical standalone dependency pins.

```sh
vanllia check
vanllia plan --unit pci
vanllia build --unit pci
vanllia test -- pci.inspection
```

The minimal declaration/initial-instance order was System, Hub, PCI. Its actual
initrd SHA-256 was
`03ab2470ec17ed4dbeb2c653fa6eec6a87176e4d6d25d3f5e2751f88467eca54`.
The combined PCI ELF was 322664 bytes, SHA-256
`d5c1494ead2f36216feceffd8e3e9f73a731ae08d60bc6cd0c501874255603d9`.

The native test was rerun from its final `tests/inspection` location on
single-core QEMU 10.0.13 and explicit OpenSBI 1.6 input. It observed PCI readiness,
resolved the public marked configuration endpoint, read a valid nonempty PCI
configuration snapshot, checked EOF at 65536, and released the endpoint.
Vanllia's typed native verdict required probe Reap/zero, acknowledged retirement
of probe/PCI/Hub, System completion and VM exit zero; all passed. No host-forced
stop supplied a passing result. The preceding temporary-probe run also passed
but has different bytes and is not the final-location receipt below.

Final-location initrd SHA-256:
`6a4aacecd7dee663023798fbdcf66e313ffc226abb60e35ebefbedbdae6a3150`.
Kernel SHA-256:
`c81e72b970860afff2300cc8015c831fe17c5df3a014582655eb2e4c3991fd1a`.
Firmware SHA-256:
`8f34d231fde9554e1a3c511bb1efac489da310ab0462504c512994601e12d213`
(273472 bytes). No firmware binary is copied into this provider.

The generated [receipt](docs/pci-unit-validation.json) contains the final code
hashes, exact selected sources, native verdict and serial hash. The
[serial evidence](docs/pci-inspection-serial.log) preserves the actual native
frames. Independent namer/worker/reviewer roles reviewed the declaration and
probe; the reviewer separately reran all three new host declaration cases.
No GitHub PR approval was submitted.

This proves QEMU startup, trusted supply intake, read-only inspection and native
cleanup for this composition. It does not test BAR placement, MEMORY/MASTER
effects, IOMMU, DMA or physical hardware. Effects require the authenticated
`service/device/coord` publisher, absent from this minimal composition.
Publishing the provider does not update Vanllia's or Driver's source pins;
their coordinated intake is a separate combination change.
