# VectorStrike source checkpoint

VectorStrike is a local Rust vector illustration editor built from the full upstream VectorCraft workspace at commit `231a97dd51a8dc39f62449ee81bc490f7ba9fe5c`. It retains the donor document engine, command history, renderer, SVG support, egui editor, CLI, web source, internal crates, and tests. The Windows desktop binary target is `vectorstrike`; internal crate names still use `vectorcraft-*`.

This is a source checkpoint. The previously reviewed Windows executable had SHA-256 `12883F56BFCD10565D5676D3E2776298D61FA74A3D9A07D52AB15F7BF8B2AC42`. This candidate changes its Windows version-resource text, so a new binary will have a different hash and needs its own verification. The [provenance note](STRIKE_PROVENANCE.md) describes the source; local freeze and acceptance evidence are not included in this checkout. No clean-checkout build, general cross-platform acceptance, or release package has been verified for this candidate.

## Build and verification

The root `Cargo.lock` pins the dependency graph. The root manifest declares Rust 1.95 as its minimum. A fresh checkout needs ordinary crates.io access; `--offline` works only after the registry cache is populated. Leave `CRAFT_FONTS_DIR` unset for a build using only files in this repository.

```powershell
cargo metadata --locked --no-deps --format-version 1
cargo test --locked -p vectorcraft-engine --test vectorstrike_workflow
cargo build --locked -p vectorcraft --bin vectorstrike
```

The focused workflow creates a synthetic document, edits it, verifies undo and redo, saves and reopens `.vstrike`, exports SVG, and checks malformed-document refusal. The app's control channel requires explicit authentication and scoped read/write roots. The independent operation review covers those controls and queued save completion for the frozen Windows binary; it does not replace a clean-checkout build and test.

## Source terms and attribution

Original upstream VectorCraft code is provided under its MIT OR Apache-2.0 terms. Keep [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE), and [`NOTICE`](NOTICE) with that code. The new VectorStrike modifications and temporary V-shaped UI mark have **no newly granted first-party license stated in this checkout**. The mark is a placeholder, not an approved final logo. The removed donor dragon icon and ArtCraft trademark artwork are not included.

The root workspace retains the upstream SPDX and repository metadata. The 22 unchanged donor packages keep their original inherited SPDX license field. The five packages containing local VectorStrike edits omit a package-level license field; their original upstream portions retain the donor grants. The Windows version resource names the original upstream copyright and refers to `NOTICE` for VectorStrike changes without asserting a combined-code license.

[`ASSETS.md`](ASSETS.md) identifies retained fonts, SVG icons, and example art and points to their notices. Internal package names and retained upstream copyright remain attributed to VectorCraft authors; they do not assert that the donor work was authored by VectorStrike.

## Packaging status

Legacy VectorCraft bundle, installer, signing, and platform package commands are **unavailable for VectorStrike**. In particular, do not run `cargo xtask bundle` or the scripts under `packaging/` as VectorStrike release steps: they still require removed donor artwork and donor product identifiers. [`packaging/README.md`](packaging/README.md) lists the boundary. No replacement app icon, installer, or release has been approved.
