//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. We use the `cargo:rustc-env=VAR=VALUE` syntax
    // to set environment variables for the compiler.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Set the environment variable `TEST_FOO` with the current timestamp
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. We use the `cargo:rustc-cfg=feature="name"` syntax
    // to enable a feature.
    // Enable the "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
