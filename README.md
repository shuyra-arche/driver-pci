# PCI Unit

PCI owns bounded ECAM enumeration, read-only configuration snapshots,
authenticated BAR placement and MEMORY/MASTER effects in its own Task/Space.

## Layout and boundaries

- `api/`: placement and inspection messages, marks and failures.
- `server/`: private MMIO/scanning implementation, `serve()`, and the `pci` binary.
- `declaration/`: compatibility Rust policy and build coordinates.
- `unit.toml`: startup after Hub, with System ECAM/aperture supplies.
- `tests/unit-document/`: host declaration checks.
- `tests/inspection/`: optional native inspection probe.

Placement enables MEMORY only. MASTER enable/disable and release are separate
authenticated effects; caller identity comes from the Kernel. Exact BAR roots,
generation floors, partial-failure retention and disable-before-release ordering
are safety boundaries. Raw MMIO constructors stay private. PCI has no IOMMU
implementation dependency. Inspection and placement endpoints belong to the same
PCI task.

## Build and test

Install Rust through rustup; the checkout pins nightly-2026-09-04 and RV64.
Uncached dependencies require network access.

```sh
cargo build --locked -p pci-server --bin pci --target riscv64gc-unknown-none-elf --release
cargo test --locked -p pci-api --tests --target x86_64-unknown-linux-gnu
cargo test --locked -p pci-server --lib --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
```

Host checks cover protocol, bounded access and declaration behavior. The
[native inspection probe](tests/inspection/README.md) runs through
[Vanllia](https://github.com/shuyra-arche/vanllia); it does not test BAR effects,
MASTER or DMA. Vanllia owns the combined SDK overlay; standalone manifests retain
their locked dependency graph.

See [source attribution](PROVENANCE.md) and the
[organization contribution guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
