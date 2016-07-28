use std::process::Command;

#[test]
pub fn run_ctests() {
    let profile = if cfg!(profile = "release") {
        "PROFILE=release"
    } else {
        "PROFILE=debug"
    };

    match Command::new("make")
        .args(&["-C", "ctests", profile, "check"])
        .status() {
            Ok(status) if status.success() => (),
            Ok(rc) => panic!("ctests failed with status: {}", rc),
            Err(e) => panic!("failed to run ctests: {}", e)
        }
}
