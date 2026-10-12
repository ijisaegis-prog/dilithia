//! Conservative V1 discovery: inventory the complete local production source/build
//! surface, then compare exact bytes, not a regex pretending to parse Rust.
//! Any drift needs inventory review, even if unrelated to boundary identifiers.
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

const HEADER: &str = "DILITHIA_CONSENSUS_BOUNDARY_INVENTORY_V1";
const INVENTORY: &str = include_str!("../consensus_boundary_inventory_v1.txt");
const EMPTY_AND_EXCEPTIONS: &[&str] = &[
    "production_enrollments=[]",
    "typed_consumers=[]",
    "raw_byte_bypass=OPEN",
    "raw_exception|crates/dilithia-crypto/src/lib.rs|sha3_256|generic primitive; not a typed consensus boundary",
    "raw_exception|crates/dilithia-crypto/src/lib.rs|constitution_hash|SPECIFICATION.md section 4.1; exact Constitution bytes, not DCS",
    "raw_exception|crates/dilithia-crypto/src/lib.rs|constitution_hash_checked|SPECIFICATION.md section 4.1 validation then Constitution hash",
];

fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2)
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("invalid lowercase source hex".into());
    }
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).map_err(|e| e.to_string())
        })
        .collect()
}

fn parse_inventory(text: &str) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("unknown inventory schema".into());
    }
    for expected in EMPTY_AND_EXCEPTIONS {
        if lines.next() != Some(*expected) {
            return Err(format!("missing/changed inventory declaration: {expected}"));
        }
    }
    let mut sources = BTreeMap::new();
    let mut previous = String::new();
    for line in lines {
        let parts: Vec<_> = line.split('|').collect();
        if parts.len() != 3 || parts[0] != "source" {
            return Err("unknown record".into());
        }
        let path = parts[1];
        if path <= previous.as_str()
            || path.contains('\\')
            || path
                .split('/')
                .any(|p| p.is_empty() || p == ".." || p == ".")
        {
            return Err("unordered, duplicate, or unsafe path".into());
        }
        previous = path.into();
        sources.insert(path.into(), decode_hex(parts[2])?);
    }
    if sources.is_empty() {
        return Err("missing source inventory".into());
    }
    Ok(sources)
}

fn visit(root: &Path, path: &Path, sources: &mut BTreeMap<String, Vec<u8>>) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if metadata.file_type().is_symlink() {
        return Err(format!("source link: {}", path.display()));
    }
    if metadata.is_dir() {
        for child in fs::read_dir(path).map_err(|e| e.to_string())? {
            visit(root, &child.map_err(|e| e.to_string())?.path(), sources)?;
        }
    } else if metadata.is_file() {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .ok_or("non-UTF8 filename")?;
        // Includes build.rs, examples/benches and modules outside src. Integration
        // tests are not production, but their directory can still be traversed.
        let relative = path
            .strip_prefix(root)
            .unwrap()
            .to_str()
            .ok_or("non-UTF8 path")?
            .replace('\\', "/");
        let test_only = relative.split('/').any(|part| part == "tests");
        if name == "Cargo.toml" || (path.extension().is_some_and(|ext| ext == "rs") && !test_only) {
            sources.insert(relative, fs::read(path).map_err(|e| e.to_string())?);
        }
    } else {
        return Err(format!("non-file source entry: {}", path.display()));
    }
    Ok(())
}

fn compare(
    expected: &BTreeMap<String, Vec<u8>>,
    actual: &BTreeMap<String, Vec<u8>>,
) -> Result<(), String> {
    if expected.keys().collect::<Vec<_>>() != actual.keys().collect::<Vec<_>>() {
        return Err("production source/build path set drift; inventory review required".into());
    }
    for (path, bytes) in actual {
        if expected[path] != *bytes {
            return Err(format!("source drift: {path}; inventory review required"));
        }
    }
    Ok(())
}

#[test]
fn explicit_empty_inventory_matches_discovered_production_surface() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    let expected = parse_inventory(INVENTORY).unwrap();
    let mut actual = BTreeMap::new();
    visit(&root, &root.join("crates"), &mut actual).unwrap();
    for name in ["Cargo.toml", "Cargo.lock"] {
        actual.insert(name.into(), fs::read(root.join(name)).unwrap());
    }
    compare(&expected, &actual).unwrap();
    // Exact reviewed baseline has only the trait declaration before test code.
    let module =
        std::str::from_utf8(&actual["crates/dilithia-core/src/consensus_value.rs"]).unwrap();
    let production = module.split("#[cfg(test)]").next().unwrap();
    assert!(!production.contains("impl ConsensusValue for"));
    for (path, bytes) in &actual {
        if path.ends_with(".rs")
            && path != "crates/dilithia-core/src/consensus_value.rs"
            && path != "crates/dilithia-core/src/lib.rs"
        {
            let source = std::str::from_utf8(bytes).unwrap();
            for name in [
                "ConsensusValue",
                "CanonicalConsensusBytes",
                "canonical_consensus_bytes",
            ] {
                assert!(
                    !source.contains(name),
                    "unlisted boundary reference: {path}: {name}"
                );
            }
        }
    }
}

#[test]
fn discovery_rejects_additions_removals_aliases_consumers_and_build_changes() {
    let expected = parse_inventory(INVENTORY).unwrap();
    for addition in [
        "impl ConsensusValue for Unlisted {}",
        "use dilithia_core::ConsensusValue as Alias; impl Alias for Unlisted {}",
        "fn consume(b: dilithia_core::CanonicalConsensusBytes) {}",
        "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));",
        "#[cfg(feature = \"escape\")] pub mod sealed;",
    ] {
        let mut changed = expected.clone();
        changed
            .get_mut("crates/dilithia-core/src/lib.rs")
            .unwrap()
            .extend_from_slice(addition.as_bytes());
        assert!(compare(&expected, &changed).is_err(), "{addition}");
    }
    let mut added = expected.clone();
    added.insert("crates/dilithia-core/src/unlisted.rs".into(), Vec::new());
    assert!(compare(&expected, &added).is_err());
    let mut removed = expected.clone();
    removed.remove("crates/dilithia-core/src/lib.rs");
    assert!(compare(&expected, &removed).is_err());
    let mut manifest = expected.clone();
    manifest
        .get_mut("Cargo.toml")
        .unwrap()
        .extend_from_slice(b"\n# changed build graph\n");
    assert!(compare(&expected, &manifest).is_err());
}

#[test]
fn inventory_schema_and_empty_sets_fail_closed() {
    for invalid in [
        INVENTORY.replacen(HEADER, "DILITHIA_CONSENSUS_BOUNDARY_INVENTORY_V2", 1),
        INVENTORY.replacen(
            "production_enrollments=[]",
            "production_enrollments=[Unlisted]",
            1,
        ),
        INVENTORY.replacen("typed_consumers=[]", "typed_consumers=[Unlisted]", 1),
        format!("{INVENTORY}unknown_record\n"),
        format!("{INVENTORY}{}\n", INVENTORY.lines().last().unwrap()),
    ] {
        assert!(parse_inventory(&invalid).is_err());
    }
}
