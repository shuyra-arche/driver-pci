#![no_std]
//! PCI-owned host-safe declarations shared by the driver runtime and composition.

pub use unit;

pub mod bindings;
pub mod marks;
pub mod program;

pub use program::PROGRAM;

pub const BUILD: unit::ImageBuild = unit::ImageBuild {
    image: "pci",
    manifest: "server/Cargo.toml",
    package: "pci-server",
    bin: "pci",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
