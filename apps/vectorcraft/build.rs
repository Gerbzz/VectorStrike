//! Windows only: embed the app icon and version info (VERSIONINFO) into `vectorstrike.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `VECTORSTRIKE_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/branding/VectorStrike.ico");
    println!("cargo:rerun-if-env-changed=VECTORSTRIKE_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/branding/VectorStrike.ico")
        .set("ProductName", "VectorStrike")
        .set("FileDescription", "VectorStrike vector illustration editor")
        .set("CompanyName", "VectorStrike contributors")
        .set(
            "LegalCopyright",
            "Upstream VectorCraft: Copyright (c) 2026 ArtCraft Team and the VectorCraft contributors. VectorStrike changes: see NOTICE.",
        )
        .set("OriginalFilename", "vectorstrike.exe")
        .set("InternalName", "vectorstrike");
    if let Err(e) = res.compile() {
        if std::env::var_os("VECTORSTRIKE_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=vectorstrike.exe built without icon/version resources: {e}");
    }
    Ok(())
}
