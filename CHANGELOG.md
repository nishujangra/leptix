# Changelog

All notable changes to this project are documented in this file.

## [v0.1.1] - 2026-02-16

### Added

- `Input` component in `src/components/input.rs` with:
  - Optional `class`
  - Optional `placeholder`
  - Optional `r#type` (defaults to `text`)
  - Optional two-way `model` (`Signal<String>`, `WriteSignal<String>`)
- Input styles in `src/styles/input.css`.
- Input style export in `src/styles/mod.rs` via `INPUT_CSS`.
- Dedicated input showcase at `examples/input.rs`.

### Changed

- Component exports include `Input` through `src/components/mod.rs` and `src/lib.rs`.
- `README.md` now documents `Input` API and includes `examples/input.rs`.

## [v0.1.0] - 2026-02-16

Initial release on `master` branch up to commit `81efb51` (includes `2fbeab7`, `e8467bb`, and `81efb51`).

### Added

- Initial crate setup with `Cargo.toml`, `Cargo.lock`, and feature flags (`csr`, `ssr`).
- Package metadata in `Cargo.toml`:
  - `description = "Simple UI components for Leptos."`
  - `repository = "https://github.com/nishujangra/leptix"`
- `.env` in `.gitignore`.
- `Button` component in `src/components/button.rs` with:
  - Optional custom `class`
  - `disabled` support
  - Optional `on_click` callback
  - Child content support
- Style system with one-time style injection:
  - `src/style/injection.rs`
  - `src/styles/button.css`
- Public exports via `src/lib.rs` and `src/components/mod.rs`.
- Example app in `examples/basic.rs`, including the expanded button showcase page.
- MIT license (`LICENSE.md`) and project README (`README.md`).
