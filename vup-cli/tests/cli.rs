use assert_cmd::Command;

/// Test bumping well-formatted major numbers.
#[test]
fn test_bump_major_good_input() -> Result<(), Box<dyn std::error::Error>> {
    let tests = [
        ("1.2.3", "2.0.0"),
        ("2.0.0", "3.0.0"),
        ("0.2.3", "1.0.0"),
        ("5.0.5", "6.0.0"),
        ("1000.0.0", "1001.0.0"),
        ("16.1000.0", "17.0.0"),
        ("9999999999999999999.0.0", "10000000000000000000.0.0"),
        ("18446744073709551614.0.0", "18446744073709551615.0.0"),
    ];

    for test in tests {
        let (input, output) = test;

        Command::cargo_bin("vup")?
            .arg("major")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }

    Ok(())
}

/// Test bumping well-formatted minor numbers.
#[test]
fn test_bump_minor_good_input() -> Result<(), Box<dyn std::error::Error>> {
    let tests = [
        ("1.2.3", "1.3.0"),
        ("2.0.0", "2.1.0"),
        ("0.2.3", "0.3.0"),
        ("5.0.5", "5.1.0"),
        ("1000.0.0", "1000.1.0"),
        ("1.18446744073709551614.18", "1.18446744073709551615.0"),
    ];

    for test in tests {
        let (input, output) = test;

        Command::cargo_bin("vup")?
            .arg("minor")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }

    Ok(())
}

/// Test bumping well-formatted patch numbers.
#[test]
fn test_bump_patch_good_input() -> Result<(), Box<dyn std::error::Error>> {
    let tests = [
        ("1.2.3", "1.2.4"),
        ("2.0.0", "2.0.1"),
        ("0.2.3", "0.2.4"),
        ("5.0.5", "5.0.6"),
        ("1000.0.0", "1000.0.1"),
        ("1.1.18446744073709551614", "1.1.18446744073709551615"),
    ];

    for test in tests {
        let (input, output) = test;

        Command::cargo_bin("vup")?
            .arg("patch")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(output);
    }

    Ok(())
}

/// Test bumping `u64` max fails.
#[test]
fn test_bump_overflow() -> Result<(), Box<dyn std::error::Error>> {
    for bump in ["major", "minor", "patch"] {
        Command::cargo_bin("vup")?
            .arg(bump)
            .write_stdin(concat!(
                "18446744073709551615.",
                "18446744073709551615.",
                "18446744073709551615",
            ))
            .assert()
            .failure();
    }

    Ok(())
}

/// Test extra dot characters in the version string fail.
#[test]
fn test_extra_dots() -> Result<(), Box<dyn std::error::Error>> {
    let tests = ["1..1.1", "1.1..1", "1.1.1.", ".1.1.1", "..1.1.1", "1..1..1"];

    for test in tests {
        for bump in ["major", "minor", "patch"] {
            Command::cargo_bin("vup")?
                .arg(bump)
                .write_stdin(test)
                .assert()
                .failure();
        }
    }

    Ok(())
}

/// Test negative numbers fail.
#[test]
fn test_negative_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let tests = ["-1.1.1", "1.-1.1", "1.1.-1", "-0.1.1"];

    for test in tests {
        for bump in ["major", "minor", "patch"] {
            Command::cargo_bin("vup")?
                .arg(bump)
                .write_stdin(test)
                .assert()
                .failure();
        }
    }

    Ok(())
}

/// Test random characters in version numbers fails.
#[test]
fn test_non_digits() -> Result<(), Box<dyn std::error::Error>> {
    let tests = ["a.b.c", "abc", "---", "x.1.a", "1F.C.8A"];

    for test in tests {
        for bump in ["major", "minor", "patch"] {
            Command::cargo_bin("vup")?
                .arg(bump)
                .write_stdin(test)
                .assert()
                .failure();
        }
    }

    Ok(())
}

/// Test for failure on an empty standard input.
#[test]
fn test_empty_stdin() -> Result<(), Box<dyn std::error::Error>> {
    for bump in ["major", "minor", "patch"] {
        Command::cargo_bin("vup")?.arg(bump).assert().failure();
    }

    Ok(())
}

/// Test numbers with leading 0s fail.
#[test]
fn test_leading_0s() -> Result<(), Box<dyn std::error::Error>> {
    let tests = [
        "01.1.1",
        "000000000001.1.1",
        "1.01.1",
        "1.1.01",
        "1.000000000000000000000000000000000000000000000000001.1",
        "00.1.1",
    ];

    for test in tests {
        for bump in ["major", "minor", "patch"] {
            Command::cargo_bin("vup")?
                .arg(bump)
                .write_stdin(test)
                .assert()
                .failure();
        }
    }

    Ok(())
}
