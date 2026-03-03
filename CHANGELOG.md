# Changelog

All notable changes to this project are documented in this file.

## [v0.1.2] - 2026-03-03

### Added

- `Toast` component in `src/components/toast.rs` with:
  - `ToastVariant` enum: Success, Error, Warning, Info
  - `ToastPosition` enum: TopLeft, TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight
  - Optional `message` (required text content)
  - Optional `variant` (defaults to Info)
  - Optional `position` (defaults to TopRight)
  - Optional `is_open` for visibility control
  - Optional `on_close` callback for dismiss events
  - Optional `duration` for auto-dismiss functionality (in milliseconds)
  - Optional `class` for custom CSS classes
- Toast styles in `src/styles/toast.css` with:
  - Color variants (success green, error red, warning yellow, info blue)
  - Position-based fixed positioning
  - Slide-in/out animations
  - CSS custom properties for theme customization
- Toast style export in `src/styles/mod.rs` via `TOAST_CSS`
- Dedicated toast showcase at `examples/toast.rs` with 4 sections:
  - Toast Variants demonstration
  - Positioning showcase (all 6 positions)
  - Auto-Dismiss feature with 2s, 5s, 10s examples
  - Interactive Control with custom message and configuration
- `ButtonVariant` enhancement to `Button` component in `src/components/button.rs`:
  - `ButtonVariant` enum: Primary, Secondary, Danger, Outline
  - Optional `variant` prop (defaults to Primary)
  - CSS classes for each variant with hover states
- Enhanced button styles in `src/styles/button.css` with variant-specific colors
- Comprehensive Rust documentation (///) for all components:
  - Full component descriptions
  - Props documentation
  - Usage examples
  - Enum variant descriptions
- Updated `examples/basic.rs` to showcase Button variants
- Updated `examples/input.rs` with module-level documentation
- `gloo-timers` dependency (v0.3) with futures feature for async auto-dismiss

### Changed

- `Cargo.toml` version bumped to 0.1.2
- Component exports now include `Toast`, `ToastVariant`, `ToastPosition`
- Button component now supports visual variants via CSS classes

### Dependencies Added

- `gloo-timers = { version = "0.3", features = ["futures"] }` for async timing in Toast auto-dismiss

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
