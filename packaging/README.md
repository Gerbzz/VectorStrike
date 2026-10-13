# Legacy packaging is unavailable for VectorStrike

The files in this directory are retained upstream VectorCraft packaging templates and scripts. They are **not supported VectorStrike bundle, installer, signing, or release commands**. Do not run `cargo xtask bundle` or the platform `package`, `sign`, `flatpak-bundle`, and icon-generation commands for VectorStrike. Several still require the omitted donor dragon icon or DMG art, and the rest retain VectorCraft product identifiers.

`packaging/macos/Info.plist.in` remains because Rust source and xtask include it at compile time. The Linux desktop and metainfo templates remain for existing source tests. New VectorStrike packaging needs separately approved and attributed platform art, corrected product identifiers, and its own verification before any release.
