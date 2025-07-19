use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::Builder;

#[test]
fn test_successful_generation_with_defaults() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    let temp_file = Builder::new().suffix(".png").tempfile().unwrap();
    let output_path = temp_file.path();

    cmd.arg("--width")
        .arg("200")
        .arg("--height")
        .arg("150")
        .arg("--output")
        .arg(output_path);

    cmd.assert().success();
    assert!(output_path.exists(), "Output file was not created");

    let img = image::open(output_path).unwrap();
    assert_eq!(img.width(), 200);
    assert_eq!(img.height(), 150);
}

#[test]
fn test_custom_output_format_jpeg() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("test_image.jpg");

    cmd.arg("-w")
        .arg("100")
        .arg("--height")
        .arg("100")
        .arg("-o")
        .arg(&output_path)
        .arg("-f")
        .arg("jpeg");

    cmd.assert().success();
    assert!(output_path.exists(), "JPEG output file was not created");

    let img = image::open(&output_path).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 100);
}

#[test]
fn test_custom_colors_and_text() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    let temp_file = Builder::new().suffix(".png").tempfile().unwrap();
    let output_path = temp_file.path();

    cmd.arg("--width")
        .arg("80")
        .arg("--height")
        .arg("80")
        .arg("--bg")
        .arg("#ff0000")
        .arg("--fg")
        .arg("#00ff00")
        .arg("--text")
        .arg("Hello")
        .arg("--output")
        .arg(output_path);

    cmd.assert().success();
    assert!(
        output_path.exists(),
        "File with custom colors/text was not created"
    );

    // Verifying the color would be more complex, but we can check dimensions.
    let img = image::open(output_path).unwrap();
    assert_eq!(img.width(), 80);
    assert_eq!(img.height(), 80);
}

#[test]
fn test_missing_required_width_arg() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    cmd.arg("--height").arg("100");
    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));
}

#[test]
fn test_missing_required_height_arg() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    cmd.arg("--width").arg("100");
    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));
}

#[test]
fn test_invalid_hex_code() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    cmd.arg("-w")
        .arg("100")
        .arg("--height")
        .arg("100")
        .arg("--bg")
        .arg("invalidcolor");

    // The app panics on invalid hex, which assert_cmd treats as a failure.
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid hex color"));
}

#[test]
fn test_unsupported_format() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    cmd.arg("-w")
        .arg("100")
        .arg("--height")
        .arg("100")
        .arg("-f")
        .arg("tiff");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported format"));
}

#[test]
fn test_help_message() {
    let mut cmd = Command::cargo_bin("holderplace").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Placeholder image generator"));
}
