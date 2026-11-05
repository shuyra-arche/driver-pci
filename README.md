# PCI Driver

PCI now owns its standalone runtime: bounded ECAM enumeration, read-only
configuration snapshots, authenticated BAR placement/grants and MEMORY/MASTER
effects in its own Task/Space. Package `pci-server` builds bin `pci`; the loader
image identity remains `pci`. `pci_declaration::BUILD` is the single coordinate
for `server/Cargo.toml`, package `pci-server`, bin `pci`.

`unit.toml` is the current Vanllia runtime declaration. It retains supervisor
execution, ordered effect/config capabilities, Hub startup and retirement
dependencies, and the two exact System-supplied ECAM/aperture bindings. System
completes Operator/Identity bootstrap before starting the ordinary Unit closure.
Select `pci` through Vanllia; the minimal closure is System, Hub and PCI.

Vanllia's selected-source Cargo overlay supplies the current Env/Wire/Mold/Unit
SDK together, matching the already migrated providers. Standalone manifests and
their formal lock retain the published historical dependency identities; changing
one facade alone would split its transitive ABI types. The runtime's existing
`main` and `serve()` entries need no additional wrapper or platform provider.

The existing Place/Enable/Disable/Release protocol and PROGRAM are unchanged.
`pci_api::inspection::{Call, Scan, Page, ENTRY, ENTRY_PATH, BACK}` publishes the
frozen read-only protocol (Scan: 12 encoded bytes in a 16-byte buffer; Page:
265 bytes). Actual scanning/MMIO stays in `server/`, never in the API crate.

Runtime implementation modules are private; `pci_server::serve()` is the only
library entry. Raw MMIO constructors and scanner/state types are not exposed to
external safe callers.

The server reuses `driver-shared::context` at
`bb3427e179e87b640e0d51115a44544fae231252`; only missing platform-window intake
and trusted System-state lookup are private adapters. PCI has no IOMMU
implementation dependency. Placement opens MEMORY only; MASTER remains a
separate authenticated action. Frozen generation floors, exact BAR roots,
partial-failure retention and disable-before-release rules are preserved.

## Current build and interface checks

```sh
cargo build --locked --offline -p pci-server --bin pci --target riscv64gc-unknown-none-elf --release
cargo test --locked --offline -p pci-api --tests --target x86_64-unknown-linux-gnu
cargo test --locked --offline -p pci-server --lib --target x86_64-unknown-linux-gnu
cargo test --locked --offline --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
```

The original runtime checks used nightly-2026-09-04; the Unit intake is also
checked with Vanllia's nightly-2026-10-09. Prepare public dependency caches first.
The guest linker script is applied only to freestanding targets. Host checks
exercise wire/data and bounded configuration access, not the guest binary or
real hardware. The isolated Unit tests check TOML policy against the retained
Rust declaration, binary record/runtime conversion, and full-width marks. No old
test suite or guest scene is ported. These host/build checks do not establish
hardware/DMA/retirement runtime acceptance.

The new native [`pci.inspection` smoke test](tests/inspection/README.md) selects
System, Hub, PCI and a transient probe. Through Vanllia it checks public
read-only configuration and native cleanup on QEMU; it is separate from the
host checks and exercises no BAR, MASTER or DMA effects.

Runtime source is frozen sqware commit
`94e114614bec53424ac55e640fc80cf19a3e7107`; see the appended runtime section in
[PROVENANCE.md](PROVENANCE.md). The original API snapshot remains recorded
below and in Git history. No source license grant is invented, no firmware is
copied, and no old source data or repository is deleted or archived.

## Original API snapshot (historical)

Contract-only snapshot of the Shuyra PCI declarations and public protocol, plus
the three existing pure protocol tests.

This repository is the public PCI provider prerequisite for later Kernel and
Source consumers. It is **not** a standalone PCI Driver Unit, server, runtime,
image build, or composition catalog. It contains no hardware access, no loader,
no Unit runtime, and no `prog-pci` image target. Source (`programs`,
`crates/composition`) still owns the driver body and the actual `prog-pci`
image target until the full driver migration.

The Kernel-owned DMA-confirmation tests genuinely need the real PCI API and
declaration when their Source phase moves to public packages. That partial
closure is why this provider exists; it is not a claim that the PCI Driver Unit
has moved.

## Layout

- `declaration/` — `pci-declaration` 0.1.0. The one `UnitFile` declaration
  (`PROGRAM`, name `pci`), the ECAM/aperture supply bindings, and marks.
- `api/` — `pci-api` 0.1.0. `Place`, `Toggle`, `Said`, `Wire`, `Call`, the
  generated `Fail` code table, and the entry/back/BAR marks and action codes.
  API sibling paths such as `../declaration` stay inside this provider.
- `api/tests/protocol.rs` — the three existing Source tests, byte-identical:
  request/reply golden bytes and bounds, malformed strictness, and
  back-token/mark/fail-code checks.

Each ordinary data type remains in its own file. No production Rust file was
refactored, renamed, reformatted, visibility-widened, or given a new facade.

## Contract preserved

`PLACE`, `ENABLE`, `DISABLE`, and `RELEASE` keep their existing effect
meaning: placement opens memory space only; enable/disable MASTER is separate;
release removes the placement. The messages carry no trusted caller field —
sender identity is supplied by the real Kernel, never by the new caller. The
retained golden sizes are `Place` 22 encoded / 24 buffer, `Toggle` 21 encoded /
24 buffer, `Said` 13 encoded / 16 buffer. Messages are not padded to buffer
capacity.

The host cases exercise data types and wire bytes only. They do not prove
Kernel sender authentication, hardware effects, or driver lifetime behavior;
those remain in Source/Kernel tests until their own phases.

## Public pins

| Component | Repository | Revision |
| --- | --- | --- |
| `env` | `https://github.com/shuyra-arche/env.git` | `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae` |
| `unit` | `https://github.com/shuyra-arche/unit.git` | `90396f0fda376e0693fb1b9553aa21864d241057` |
| `system-api` | `https://github.com/shuyra-arche/system.git` | `9e5d63ca536a176583cf1b0054b4ecf7ddd88257` |
| `wire` | `https://github.com/shuyra-arche/wire.git` | `0ff77dc4573fa2620d2fe995a18c01949d9c7e57` |
| `mold` | `https://github.com/shuyra-arche/mold.git` | `f52501f4c91b686eded8b4bcd743b4e96084b7f9` |

There is no PCI-to-IOMMU dependency. The two providers can be published in
either order.

## Build and test

```sh
cargo build --locked --target x86_64-unknown-linux-gnu --lib
cargo build --locked --target riscv64gc-unknown-none-elf --lib
cargo build --locked --release --target riscv64gc-unknown-none-elf --lib
cargo test -p pci-api --locked --target x86_64-unknown-linux-gnu
```

Add `--offline` after the dependency cache is prepared. The API packages set
`test = false` on the library, so the three cases execute through the
integration test target, not as library unit tests.

The test target's direct dependencies are `env`, `wire`, `pci-api`, and
`pci-declaration` — a subset of the production closure. The full 13-package
provider closure is 2 local packages plus `env`, `unit`, `system-api`, `wire`,
`mold`, `bitflags`, `erra`, `proc-macro2`, `quote`, `syn`, and `unicode-ident`;
there are no test-only external dependencies.

## Provenance and license

See `PROVENANCE.md` for the frozen Source commit, authors, old-to-new file
hashes, test mapping, and exact limits. Source had no tracked license or
manifest license/authors field; this snapshot adds none and asserts no
copyright permission.
