# README screenshot provenance

The three README images are direct `ui.screenshot` captures of the retained native VectorStrike Windows executable on 2026-10-14. They have not been composited, retouched, or redrawn. The original “Tidal Forms” sample was made in the app for this documentation by creating rectangles, ellipses, closed paths, two text objects, and a second layer. No downloaded artwork, client asset, or proprietary-app reference was used.

The executable's SHA-256 is `89F4B228883FE6845459C33995F23E76FB361A0B50CD5D03A105EB508C56A865`. Its Cargo depfile points to a separate publication-candidate checkout. Comparing that checkout with the later local source tree found **zero missing** and **zero changed Rust or lock files** across 1,131 code-related files. Six Cargo/version-resource files differed only in license metadata and copyright wording. Checked files from the public repository, including the app entry point, UI control implementation, lockfile, and all six differing build metadata files, match the publication-candidate checkout. This is source-match evidence for the illustrated behavior, not a clean-checkout rebuild or full acceptance test.

The app ran with preferences disabled, a fresh authenticated localhost control token, and read/write roots restricted to an isolated task folder. The sample `.vstrike` project was saved there and screened for local paths and external URLs. The first screenshot shows that saved warm-color project. For the second, one ellipse was selected and its fill changed from yellow to mint; the visible selection and unsaved marker are native UI. For the third, the Layers tab was opened, the selection cleared, and the window was resized before fitting the artboard. The app's path-bearing save status was cleared through its control channel before capture to avoid publishing a local scratch path. No screenshot pixels were altered afterward.

The final capture folder used about 1.3 MiB. No builds, downloads, or deletions were performed for these captures.

| Asset | Size | SHA-256 |
| --- | ---: | --- |
| `vectorstrike-workspace.png` | 318,569 bytes | `B44A11F00033C23F9B18E263C42A83B17DEF720311FA8384C8897854449E9163` |
| `vectorstrike-recolor.png` | 327,008 bytes | `2E73C8620A9C396D33D11F06DC22E8B0DD8CD12BF1F62485ABAFF252853A7FBC` |
| `vectorstrike-layers.png` | 212,366 bytes | `E9F4653020314C8ED93F191BDC35451C8543FCE76ABC97D90DF734FB915A1988` |
