//! Ephemeral downstream packages; no fixture is a workspace member or source authority.
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture {
    id: &'static str,
    body: String,
    expected: Option<(&'static str, &'static str, &'static str)>,
}

fn rejected_field(id: &'static str, ty: &str, diagnostic_type: &'static str) -> Fixture {
    Fixture {
        id,
        body: format!(
            "use dilithia_core::CanonicalConsensusField;\nfn require<T: CanonicalConsensusField>() {{}}\nfn main() {{ require::<{ty}>(); }}\n"
        ),
        expected: Some((
            "CANONICAL_FIELD_NOT_CERTIFIED",
            "CanonicalConsensusField",
            diagnostic_type,
        )),
    }
}

#[test]
fn downstream_capabilities_and_ordinary_mechanics() {
    let core = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    let repo = core.parent().unwrap().parent().unwrap();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let run = repo
        .join("target")
        .join("determinism-boundary-fixtures")
        .join(format!("{}-{nonce}", std::process::id()));
    // Artifacts are naturally discarded with target; no deletion of user files.
    fs::create_dir_all(&run).unwrap();
    let mut fixtures = vec![
        Fixture {
            id: "ordinary_mechanics",
            body: "use dilithia_core::{CanonicalConsensusBytes, CanonicalConsensusField, ConsensusValue};\nfn require<T: CanonicalConsensusField>() {}\n#[allow(dead_code)]
fn view(bytes: &CanonicalConsensusBytes) -> &[u8] { bytes.as_bytes() }\n#[allow(dead_code)]
fn generic<V: ConsensusValue>(v: &V) { let bytes = v.canonical_consensus_bytes(); assert!(!view(&bytes).is_empty()); }\nfn main() { let a = [10_u8, 20]; let i: usize = 1; let delta: isize = -1; let j = i.checked_add_signed(delta).unwrap(); assert_eq!(a[j], 10); require::<Option<Option<[u8; 0]>>>(); require::<core::primitive::u64>(); }\n".into(),
            expected: None,
        },
        Fixture {
            id: "external_value_impl",
            body: "use dilithia_core::ConsensusValue;\nstruct Downstream;\nimpl ConsensusValue for Downstream {}\nfn main() {}\n".into(),
            expected: Some(("SEALED_CAPABILITY_EXTERNAL_IMPL_DENIED", "sealed::Value", "Downstream")),
        },
        rejected_field("usize", "usize", "usize"),
        rejected_field("isize", "isize", "isize"),
        rejected_field("f32", "f32", "f32"),
        rejected_field("f64", "f64", "f64"),
        // Fixed-width signed integers are not DCS primitives and remain uncertified.
        rejected_field("i8", "i8", "i8"),
        rejected_field("i16", "i16", "i16"),
        rejected_field("i32", "i32", "i32"),
        rejected_field("i64", "i64", "i64"),
        rejected_field("i128", "i128", "i128"),
        rejected_field("option_usize", "Option<usize>", "usize"),
        rejected_field("vec_usize", "Vec<usize>", "Vec<usize>"),
        // Dynamic byte vectors stay uncertified until a normative bounded collection rule exists.
        rejected_field("vec_u8_unbounded", "Vec<u8>", "Vec<u8>"),
        rejected_field("qualified_usize", "core::primitive::usize", "usize"),
        rejected_field("nested_option_usize", "Option<Option<usize>>", "usize"),
    ];
    let mut alias = rejected_field("alias_count", "Count", "usize");
    alias.body.insert_str(0, "type Count = usize;\n");
    fixtures.push(alias);

    // Negative-only shape controls: these prove only that unregistered external
    // composites are not certified fields. They are not positive recursive
    // composition coverage and define no future struct/enum/tuple encoding.
    let mut unregistered_struct = rejected_field(
        "unregistered_struct_field",
        "NeutralStruct",
        "NeutralStruct",
    );
    unregistered_struct.body.insert_str(
        0,
        "#[allow(dead_code)]\nstruct NeutralStruct { first: u64, second: u16 }\n",
    );
    fixtures.push(unregistered_struct);

    let mut unregistered_enum =
        rejected_field("unregistered_enum_field", "NeutralEnum", "NeutralEnum");
    unregistered_enum.body.insert_str(
        0,
        "#[allow(dead_code)]\nenum NeutralEnum { First(u64), Second(u16) }\n",
    );
    fixtures.push(unregistered_enum);
    fixtures.push(rejected_field(
        "unregistered_tuple_field",
        "(u64, u16)",
        "(u64, u16)",
    ));

    // Use distinctive generic names so the harness's diagnostic substring
    // checks cannot receive credit from an unrelated single-letter `T`.
    fixtures.push(Fixture {
        id: "unconstrained_generic_field",
        body: "use dilithia_core::CanonicalConsensusField;\nfn require<Field: CanonicalConsensusField>() {}\nfn generic<UnprovenFieldType>() { require::<UnprovenFieldType>(); }\nfn main() {}\n".into(),
        expected: Some((
            "CANONICAL_FIELD_UNCONSTRAINED_GENERIC_DENIED",
            "CanonicalConsensusField",
            "UnprovenFieldType",
        )),
    });
    fixtures.push(Fixture {
        id: "recursive_generic_bound",
        body: "use dilithia_core::CanonicalConsensusField;\nfn require<Field: CanonicalConsensusField>() {}\nfn generic<UnprovenInnerType>() { require::<Option<UnprovenInnerType>>(); }\nfn main() {}\n".into(),
        expected: Some((
            "CANONICAL_FIELD_RECURSIVE_GENERIC_BOUND_DENIED",
            "CanonicalConsensusField",
            "UnprovenInnerType",
        )),
    });

    // TEMPORARY implementation-state guard: remove or revise when a normative
    // Specification 짠3.6 String byte limit and reviewed String certification exist.
    fixtures.push(rejected_field("string_withheld", "String", "String"));
    let core_path = core
        .to_str()
        .unwrap()
        .trim_start_matches(r"\\?\")
        .replace('\\', "/");
    let mut results = String::from("fixture\texpected_class\tresult\n");
    for fixture in fixtures {
        let package = run.join(fixture.id);
        fs::create_dir_all(package.join("src")).unwrap();
        let manifest = format!(
            "[package]\nname = \"boundary_{}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[dependencies]\ndilithia-core = {{ path = \"{}\" }}\n",
            fixture.id, core_path
        );
        fs::write(package.join("Cargo.toml"), manifest).unwrap();
        fs::write(package.join("src/main.rs"), &fixture.body).unwrap();
        let output = Command::new(env!("CARGO"))
            .args([
                "check",
                "--offline",
                "--color",
                "never",
                "--message-format=short",
                "--manifest-path",
            ])
            .arg(package.join("Cargo.toml"))
            .arg("--target-dir")
            .arg(run.join("cargo-target"))
            .current_dir(&package)
            .output()
            .expect("fixture Cargo must execute; infrastructure failure gives no credit");
        fs::write(package.join("stdout.txt"), &output.stdout).unwrap();
        fs::write(package.join("stderr.txt"), &output.stderr).unwrap();
        let stderr = String::from_utf8(output.stderr).expect("UTF-8 compiler diagnostics");
        if let Some((class, capability, ty)) = fixture.expected {
            assert!(
                !output.status.success(),
                "{}: {class}: unexpectedly compiled",
                fixture.id
            );
            // Every actual error must be the intended local trait-bound error.
            // No syntax/import/dependency failure may borrow credit from E0277.
            let errors: Vec<_> = stderr
                .lines()
                .filter(|line| line.contains("error["))
                .collect();
            assert!(
                !errors.is_empty(),
                "{}: {class}: no coded diagnostic: {stderr}",
                fixture.id
            );
            assert!(
                errors
                    .iter()
                    .all(|line| line.replace('\\', "/").contains("src/main.rs:")
                        && line.contains("error[E0277]")
                        && line.contains(capability)
                        && line.contains(ty)),
                "{}: {class}: unrelated diagnostic: {stderr}",
                fixture.id
            );
            assert!(
                !stderr.lines().any(|line| line.starts_with("error:")
                    && !line.starts_with("error: could not compile")),
                "{}: infrastructure error: {stderr}",
                fixture.id
            );
            if fixture.id == "recursive_generic_bound" {
                // The existing short diagnostic intentionally proves the local E0277
                // and names the distinctive inner generic. Re-run only this fixture
                // with the full diagnostic renderer to prove the failure arose through
                // Option<T>'s recursive CanonicalConsensusField bound.
                let detailed = Command::new(env!("CARGO"))
                    .args(["check", "--offline", "--color", "never", "--manifest-path"])
                    .arg(package.join("Cargo.toml"))
                    .arg("--target-dir")
                    .arg(run.join("cargo-target"))
                    .current_dir(&package)
                    .output()
                    .expect("recursive-bound detailed Cargo diagnostic must execute");
                assert!(
                    !detailed.status.success(),
                    "{}: recursive-bound detailed check unexpectedly compiled",
                    fixture.id
                );
                let detailed_stderr = String::from_utf8(detailed.stderr)
                    .expect("UTF-8 detailed compiler diagnostics");
                assert!(
                    detailed_stderr.contains(
                        "required for `Option<UnprovenInnerType>` to implement `CanonicalConsensusField`"
                    ),
                    "{}: recursive-bound detailed diagnostic did not prove Option<T> propagation: {detailed_stderr}",
                    fixture.id
                );
            }
            results.push_str(&format!(
                "{}\t{class}\tPASS_EXPECTED_REJECTION\n",
                fixture.id
            ));
        } else {
            assert!(
                output.status.success(),
                "{}: COMPILE_PASS_CONTROL failed: {stderr}",
                fixture.id
            );
            results.push_str(&format!("{}\tCOMPILE_PASS_CONTROL\tPASS\n", fixture.id));
        }
        fs::write(run.join("results.tsv"), &results).unwrap();
    }
    println!("{}", results);
    println!("ephemeral fixture evidence: {}", run.display());
}
