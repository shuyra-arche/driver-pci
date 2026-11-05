#[cfg(test)]
mod tests {
    use pci_declaration::{BUILD, PROGRAM};
    use unit::{document as d, record};

    const TOML: &str = include_str!("../../unit.toml");

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(TOML).unwrap()
    }

    fn assert_policy<S: AsRef<str>>(value: &d::Unit<S>) {
        assert_eq!(value.schema, 1);
        assert_eq!(value.id.as_ref(), PROGRAM.name);
        assert_eq!(value.loader.image.as_ref(), PROGRAM.loader.image);
        assert_eq!(PROGRAM.loader.space, env::ProgramKind::Supervisor);
        assert_eq!(value.loader.space, d::Space::Supervisor);
        assert_eq!(value.loader.stack, PROGRAM.loader.stack as u64);
        assert!(PROGRAM.loader.args.is_empty());
        assert!(value.loader.args.is_empty());

        assert_eq!(
            value.operator.namespace.len(),
            PROGRAM.operator.namespace.len()
        );
        for (current, previous) in value
            .operator
            .namespace
            .iter()
            .zip(PROGRAM.operator.namespace)
        {
            assert_eq!(current.key.as_ref(), previous.key.name());
            assert_eq!(current.path.as_ref(), previous.path);
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.placement, d::Placement::Shared);
            let d::Scope::Names(names) = &current.scope else {
                panic!("namespace scope changed")
            };
            let legacy_unit::operator::Scope::Names(old_names) = previous.scope else {
                panic!("legacy scope changed")
            };
            assert_eq!(
                names.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
                old_names
            );
            assert_eq!(current.ops, previous.ops.bits());
            assert_eq!(previous.permit, Some(system_api::operator::Permit::Public));
            assert_eq!(current.permit, Some(d::Permit::Public));
            assert_eq!(current.source_mark, previous.source_mark);
            assert!(previous.bindings.is_empty());
            assert!(current.bindings.is_empty());
        }
        assert!(PROGRAM.identity.aliases.is_empty());
        assert!(value.identity.aliases.is_empty());
        assert!(PROGRAM.identity.grants.is_empty());
        assert!(value.identity.grants.is_empty());

        assert_eq!(value.control.creation, d::Creation::Startup);
        assert_eq!(
            PROGRAM.control.after.unwrap(),
            [
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Operator),
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Identity),
                legacy_unit::Dependency::Unit("hub"),
            ]
        );
        for providers in [&value.control.requires, &value.control.after] {
            assert_eq!(
                providers.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
                ["hub"]
            );
        }
        let legacy_unit::Ready::Signal(mark) = PROGRAM.control.ready else {
            panic!("legacy ready changed")
        };
        assert_eq!(value.control.ready, d::Ready::Signal(mark.get()));
        assert!(matches!(
            PROGRAM.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(value.control.ending, d::Ending::Resident);
        assert_eq!(
            value.control.supervision.as_ref().map(AsRef::as_ref),
            PROGRAM.control.supervision
        );
        assert!(PROGRAM.control.construction.is_empty());
        assert!(value.control.construction.is_empty());
        assert!(value.control.groups.is_empty());
        assert!(!value.control.guarded);
        assert_eq!(
            value.control.retirement.len(),
            PROGRAM.control.retirement.len()
        );
        for (current, previous) in value
            .control
            .retirement
            .iter()
            .zip(PROGRAM.control.retirement)
        {
            assert_eq!(current.provider.as_ref(), previous.provider);
            assert_eq!(usize::from(current.namespace), previous.namespace);
            assert_eq!(current.face.as_ref(), previous.face);
        }
        assert_eq!(value.supply.bindings.len(), PROGRAM.supply.bindings.len());
        for (current, previous) in value.supply.bindings.iter().zip(PROGRAM.supply.bindings) {
            assert_eq!(current.source.as_ref(), previous.source.name());
            assert_eq!(current.resource.as_ref(), previous.resource.name());
            assert_eq!(current.slot, previous.slot.get());
            assert_eq!(current.permission, previous.permission.bits());
        }
        value.validate().unwrap();
    }

    #[test]
    fn declaration_preserves_runtime_capability_order_and_platform_supply() {
        assert_policy(&document());
        assert_eq!(document().loader.image, BUILD.image);
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        assert_eq!(bins.get(0).unwrap()["name"].as_str(), Some(BUILD.bin));
        assert_eq!(BUILD.bin, BUILD.image);
        assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
        assert_eq!(BUILD.profile, "release");
        assert!(BUILD.default_features);
        assert!(BUILD.features.is_empty());
    }

    #[test]
    fn record_and_runtime_conversion_retain_supervisor_and_trusted_supply() {
        static BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = BYTES.get_or_init(|| record::encode(&document()).unwrap());
        let decoded = record::decode(bytes).unwrap();
        assert_policy(&decoded);
        let native = unit::runtime::UnitFile::try_from(decoded).unwrap();
        assert_eq!(native.loader.space, PROGRAM.loader.space);
        assert_eq!(
            native.control.after.as_deref(),
            Some(&[unit::Dependency::Unit("hub")][..])
        );
        assert_eq!(native.operator.namespace.len(), 2);
        for (current, previous) in native
            .operator
            .namespace
            .iter()
            .zip(PROGRAM.operator.namespace)
        {
            assert_eq!(current.key, previous.key);
            assert_eq!(current.path, previous.path);
            assert_eq!(current.ops, previous.ops);
            assert_eq!(current.permit, previous.permit);
            assert_eq!(current.source_mark, previous.source_mark);
        }
        for (current, previous) in native.supply.bindings.iter().zip(PROGRAM.supply.bindings) {
            assert_eq!(current.source, previous.source);
            assert_eq!(current.resource, previous.resource);
            assert_eq!(current.slot, previous.slot);
            assert_eq!(current.permission, previous.permission);
        }
        assert!(native.valid());
    }

    #[test]
    fn hexadecimal_marks_preserve_the_full_u64_contract() {
        let mut hexadecimal = TOML.to_owned();
        for mark in [
            pci_declaration::marks::ENTRY,
            pci_declaration::marks::CONFIG,
            pci_declaration::bindings::ECAM_BINDING.slot,
            pci_declaration::bindings::APERTURE_BINDING.slot,
            legacy_unit::READY_MARK,
        ] {
            hexadecimal = hexadecimal.replace(
                &format!("\"{}\"", mark.get()),
                &format!("\"0x{:016x}\"", mark.get()),
            );
        }
        let converted: d::Unit<String> = toml_edit::de::from_str(&hexadecimal).unwrap();
        assert_eq!(converted, document());
        assert_policy(&record::decode(&record::encode(&converted).unwrap()).unwrap());
    }
}
