//! On Windows (MSVC) the linker embeds the application manifest into every target, not tauri-build's
//! resource file, which only reaches the app binary. Without the manifest the `cargo test` harness
//! loads Common Controls v5, can't find `TaskDialogIndirect`, and exits with
//! STATUS_ENTRYPOINT_NOT_FOUND (0xc0000139) before any test runs.
//! `windows-app-manifest.xml` is a copy of tauri-build's default manifest.

use std::env;
use std::path::Path;

fn main() {
    let windows_msvc =
        env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") && env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    let mut attributes = tauri_build::Attributes::new();
    if windows_msvc {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("windows-app-manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
        // Match tauri-build's manifest exactly: no linker-generated UAC section.
        println!("cargo:rustc-link-arg=/MANIFESTUAC:NO");
        attributes = attributes.windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());
    }
    tauri_build::try_build(attributes).expect("failed to run tauri-build");
}
