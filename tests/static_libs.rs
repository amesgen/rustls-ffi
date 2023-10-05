use itertools::Itertools;
use regex::Regex;
use std::process::Command;

#[test]
#[ignore] // This test recompiles the crate and can be slow (~5s).
fn verify_static_libs() {
    // Build the crate, printing native-static-lib requirements.
    let result = Command::new("cargo")
        .args(["build", "--color", "never"])
        .env("RUSTFLAGS", "--print native-static-libs")
        .output()
        .expect("failed to execute process");
    assert!(result.status.success());

    // Search for the expected native-static-libs output in the cargo stderr.
    let re = Regex::new(r"note: native-static-libs: ([^\n]+)\n").unwrap();
    let haystack = String::from_utf8_lossy(&result.stderr);
    let native_libs = re
        .captures(&haystack)
        .expect("missing expected native-static-libs output")
        .get(1)
        .expect("missing expected native-static-libs output")
        .as_str();

    // We should find the expected native-static-libs output for the platform in question.
    // Note that we remove duplicate consecutive entries, but not duplicate entries in general
    // as these can be intended and have meaning on specific platforms.
    let actual_linker_parts: Vec<_> = native_libs.split_whitespace().dedup().collect();
    assert_eq!(
        actual_linker_parts,
        expected_linker_parts(),
        "unexpected list of static libraries. Fix or update README"
    )
}

fn expected_linker_parts() -> &'static [&'static str] {
    #[cfg(target_os = "linux")]
    {
        &[
            "-lgcc_s",
            "-lutil",
            "-lrt",
            "-lpthread",
            "-lm",
            "-ldl",
            "-lc",
        ]
    }
    #[cfg(target_os = "macos")]
    {
        &[
            "-framework",
            "Security",
            "-liconv",
            "-lSystem",
            "-lc",
            "-lm",
        ]
    }
    #[cfg(target_os = "windows")]
    {
        &[
            "advapi32.lib",
            "credui.lib",
            "kernel32.lib",
            "secur32.lib",
            "legacy_stdio_definitions.lib",
            "kernel32.lib",
            "advapi32.lib",
            "bcrypt.lib",
            "kernel32.lib",
            "ntdll.lib",
            "userenv.lib",
            "ws2_32.lib",
            "kernel32.lib",
            "ws2_32.lib",
            "kernel32.lib",
            "msvcrt.lib",
        ]
    }
}