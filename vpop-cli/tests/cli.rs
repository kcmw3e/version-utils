use assert_cmd::Command;

/// Test retrieving well-formatted version numbers and files.
#[test]
fn test_good_input() -> Result<(), Box<dyn std::error::Error>> {
    // Each test consists of 3 values: the TOML configuration, the path to the
    // version number, and the expected value.
    let tests = [
        (r#"version = "1.2.3""#, "version", "1.2.3"),
        (r#"version = "0.0.0""#, "version", "0.0.0"),
        (
            r#"version = "10231231231209124.1241321231.1024391123""#,
            "version",
            "10231231231209124.1241321231.1024391123",
        ),
        (r#"x.y.z = "1.2.3""#, "x.y.z", "1.2.3"),
        // This one is a real use-case based on Cargo workspace configurations.
        (
            r#"[workspace.package]
            version = "1.2.3""#,
            "workspace.package.version",
            "1.2.3",
        ),
    ];

    for test in tests {
        let (input, path, output) = test;

        Command::cargo_bin("vpop")?
            .arg("--path")
            .arg(path)
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }

    Ok(())
}

/// Test retrieving paths that don't exist.
#[test]
fn test_config_path_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
    // Each test consists of 3 values: the TOML configuration, the path to the
    // version number, and the expected value.
    let tests = [
        (r#"version = "1.2.3""#, "not-version"),
        (r#"package = {version = "0.0.0"}"#, "version"),
        (r#"a.b.c.e.f.g.h.i = {j = "0.0.0"}"#, "a.b.c.e.f.g.h.i.j.k"),
        (r#"a.b.c.e.f.g.h.i = {j = "0.0.0"}"#, "a.b.c.e.f.g.h.z"),
    ];

    for test in tests {
        let (input, path) = test;

        Command::cargo_bin("vpop")?
            .arg("--path")
            .arg(path)
            .write_stdin(input)
            .assert()
            .failure();
    }

    Ok(())
}

/// Test for failure on an empty standard input.
#[test]
fn test_empty_stdin() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("vpop")?.arg("--path").arg("").assert().failure();

    Ok(())
}

/// Test bad-formatted config paths fails.
#[test]
fn test_bad_format_config_paths() -> Result<(), Box<dyn std::error::Error>> {
    let tests = [
        (r#"version = "1.2.3""#, ".version"),
        (r#"package.version = "1.2.3""#, "package..version"),
        (r#"package.version = "1.2.3""#, "package,version"),
        (r#"package.version = "1.2.3""#, "package,.version"),
        (r#"package.version = "1.2.3""#, "package*.version"),
    ];

    for test in tests {
        let (input, path) = test;

        Command::cargo_bin("vpop")?
            .arg("--path")
            .arg(path)
            .write_stdin(input)
            .assert()
            .failure();
    }

    Ok(())
}

/// Test retrieving inline tables.
#[test]
fn test_inline_tables() -> Result<(), Box<dyn std::error::Error>> {
    // Each test consists of 3 values: the TOML configuration, the path to the
    // version number, and the expected value.
    let tests = [
        (r#"package = {version = "0.0.0"}"#, "package.version", "0.0.0"),
        (
            r#"[top-level]
            package = {version = "0.0.0"}"#,
            "top-level.package.version",
            "0.0.0",
        ),
    ];

    for test in tests {
        let (input, path, output) = test;

        Command::cargo_bin("vpop")?
            .arg("--path")
            .arg(path)
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }

    Ok(())
}

/// Test retrieving non-string values fails.
#[test]
fn test_non_string_values() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: THIS WHOLE FUNCTION
    // Each test consists of 3 values: the TOML configuration, the path to the
    // version number, and the expected value.
    let tests = [
        (r#"version = 1"#, "version"),
        (r#"package = {version = "0.0.0"}"#, "package"),
        (r#"temperature = 1.2341"#, "temperature"),
        (r#"is-on = false"#, "is-on"),
        (r#"timing = 2026-02-17 19:20:00"#, "timing"),
    ];

    for test in tests {
        let (input, path) = test;

        Command::cargo_bin("vpop")?
            .arg("--path")
            .arg(path)
            .write_stdin(input)
            .assert()
            .failure();
    }

    Ok(())
}
