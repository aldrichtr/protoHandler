
/// Build script for protoHandle.rs project

fn main() {
    // Only rebuild this module when there are changes to main.rs
    println!("cargo::rerun-if-changed=build/main.rs");
}
