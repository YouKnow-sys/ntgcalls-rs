use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(not(feature = "bundled"))]
compile_error!(
    "For now we only support the NTgCalls bundled, so the `bundled` feature need to be enabled"
);

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // this a hacky solution and could fail but hey, it seem to work...
    let target_dir = out_dir
        .parent() // drop `out`
        .and_then(Path::parent) // drop `libntgcalls-sys-*`
        .and_then(Path::parent) // drop `build`
        .expect("Failed to find target folder"); // we should be on {TARGET}/{PROFILE} now

    println!(
        "cargo:rustc-link-search={}",
        Path::new(&manifest_dir).join("lib").display()
    );

    #[cfg(feature = "bundled")]
    bundled(&out_dir, target_dir);
}

#[cfg(feature = "bundled")]
fn bundled(out_dir: &std::path::Path, target_dir: &std::path::Path) {
    use std::process::Command;

    let path: PathBuf;

    if let Ok(ntgcalls_path) = env::var("NTGCAllS_BUNDLE_DIR") {
        path = PathBuf::from(ntgcalls_path);
    } else {
        assert!(
            ["windows", "linux"].contains(&std::env::consts::OS),
            "Unsupported os (`{}`) for bundled lib.",
            std::env::consts::OS
        );

        assert!(
            ["x86_64", "arm64", "aarch64"].contains(&std::env::consts::ARCH),
            "Unsupported CPU architecture (`{}`) for bundled lib.",
            std::env::consts::ARCH
        );

        let url = if let Ok(ntgcalls_url) = env::var("NTGCAllS_BUNDLE_URL") {
            PathBuf::from(ntgcalls_url)
        } else {
            PathBuf::from(format!(
                "{}/ntgcalls.{}-{}-{}.zip",
                env::var("NTGCAllS_BUNDLE_URL_PREFIX").unwrap_or_else(|_| String::from(
                    "https://github.com/pytgcalls/ntgcalls/releases/download/v1.1.3" // we are currently hardcoding this version, this should change in future
                )),
                std::env::consts::OS,
                if std::env::consts::ARCH == "aarch64" {
                    "arm64"
                } else {
                    std::env::consts::ARCH
                },
                if cfg!(debug_assertions) {
                    "debug-shared_libs"
                } else {
                    "shared_libs"
                }
            ))
        };

        let curl_status = Command::new("curl")
            .args(["-LOkf", url.to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Curl is needed to download the bundled libraries!");

        if !curl_status.success() {
            panic!("Download bundled libraries from {:?} failed", url)
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let ext_status = Command::new("unzip")
            .arg(url.file_name().unwrap().to_str().unwrap())
            .current_dir(out_dir)
            .status()
            .expect("Tar is needed to upack the bundled libraries!");

        #[cfg(target_os = "windows")]
        let ext_status = Command::new("tar")
            .args(["-xzvf", url.file_name().unwrap().to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("tar is needed to upack the bundled libraries!");

        if !ext_status.success() {
            panic!("Unpack bundled libraries failed")
        }

        path = out_dir.to_owned();
    }

    println!("cargo:rustc-link-search={}", path.display());

    let file_name = if cfg!(target_os = "windows") {
        "ntgcalls.dll"
    } else if cfg!(target_os = "linux") {
        "libntgcalls.so"
    } else {
        "libntgcalls.dylib"
    };

    std::fs::copy(path.join(file_name), target_dir.join(file_name))
        .expect("failed to copy lib file to target dir.");

    println!("cargo:rustc-link-lib=ntgcalls");
}
