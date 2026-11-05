# PCI inspection smoke test

This new native Rust test uses the current Vanllia Unit and typed IPC contract.
It selects System, Hub, PCI and one transient test Unit. The probe discovers
`pci/config`, checks its native mark, reads a nonempty snapshot with a valid
vendor ID, checks EOF at function 65536, and explicitly releases its endpoint.
It expects the standard QEMU `virt` machine's present PCI host bridge.

After adding this provider to Vanllia's sources (or Driver gitlinks), use the
current Vanllia CLI and selected SDK sources:

```sh
vanllia plan --unit pci
vanllia build --unit pci
vanllia test -- pci.inspection
```

Runtime execution requires the product's explicit firmware path, SHA-256 and
length. The native verdict requires probe Reap/zero, acknowledged retirement of
every ordinary Unit, System completion and VM exit zero. Diagnostic readiness
text alone cannot pass the test.

This checks QEMU startup, trusted supply intake, public read-only inspection
and native cleanup. It does not exercise BAR placement, MASTER, IOMMU, DMA or
real hardware. No legacy test suite or scene is migrated.
