use std::{
    ffi::OsStr,
    process::{Command, ExitStatus},
};

#[test]
fn all_expand() {
    // TODO: maybe add an env var that blesses the expansions?

    macrotest::expand("tests/expand/*.rs");
}

#[test]
fn all_compile() {
    for item in std::fs::read_dir("tests/expand/").unwrap() {
        let path = item.unwrap().path();
        if path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|n| n.ends_with(".expanded.rs"))
        {
            let output = Command::new("cargo")
                .args(["+nightly", "-Zscript"])
                .arg(&path)
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();

            assert_eq!(
                output.status,
                ExitStatus::default(),
                "\x1b[1m;{}\x1b[0m; failed to compile\n stdout:\n{}\nstderr:\n{}",
                path.display(),
                str::from_utf8(&output.stdout).unwrap_or("invalid utf8"),
                str::from_utf8(&output.stderr).unwrap_or("invalid utf8")
            );
        }
    }
}

// TODO: Add a test that somehow verifies that we keep our much-faster-compile-times promise
