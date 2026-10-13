# VectorStrike source and packaging note

This local VectorStrike integration starts from `storytold/vectorcraft` commit `231a97dd51a8dc39f62449ee81bc490f7ba9fe5c`. The existing Rust document engine, command history, renderer, SVG stack, and egui desktop editor remain the foundation. Internal `vectorcraft-*` crate names identify upstream code; the desktop binary target is `vectorstrike`.

Original upstream code is MIT OR Apache-2.0. Keep `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, and applicable third-party license files with any distributed build. The original workspace SPDX and repository defaults remain for unmodified donor packages; five edited packages omit package-level license metadata while their upstream portions retain the donor grants. New VectorStrike modifications have no newly granted first-party license stated here. The V-shaped UI mark in `crates/ui-egui/src/brand.rs` is a temporary locally generated placeholder, not an approved final logo; it is not derived from the donor dragon icon.

Do not bundle `assets/app-icon/`, `docs/brand/`, or donor packaging artwork as VectorStrike assets. The Windows build script no longer embeds the donor icon. Existing packaging scripts still refer to VectorCraft and are **not** VectorStrike packages. Generate, attribute, and inspect new platform icons and update packaging before distribution.

First Windows acceptance fixture: `cargo test --locked -p vectorcraft-engine --test vectorstrike_workflow`. This creates and edits a rectangle, verifies undo/redo, saves/reopens `.vstrike`, exports SVG, and rejects a malformed document without replacing the open one. It does not establish support on other operating systems or fidelity for proprietary file formats.
