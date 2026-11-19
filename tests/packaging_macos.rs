use std::path::Path;

fn packaging(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("packaging/macos")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

#[test]
fn the_bundle_follows_the_gpu_that_drives_the_screen() {
    // Without it, a dual-GPU Intel Mac keeps ZapFastest's OpenGL context on the
    // discrete GPU, and a forced switch to the integrated one leaves every
    // window it opens blank until a relaunch.
    let plist = packaging("Info.plist");
    assert!(plist.contains("<key>NSSupportsAutomaticGraphicsSwitching</key><true/>"));
    assert!(plist.contains(
        "<key>CFBundleIdentifier</key><string>io.github.eternalwaitt.ZapFastest</string>"
    ));
    assert!(plist.contains("<key>CFBundleExecutable</key><string>zap-fastest</string>"));
}
