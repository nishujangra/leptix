# leptix

Simple UI components for Leptos.

Currently available:
- `Button`

## Compatibility

- `leptix` uses `leptos = 0.8.15`.
- Your app should also use `leptos = 0.8.15` to avoid mixed-version trait errors.
- Leptos `0.8.15` requires `rustc >= 1.88`.

---

## Status

Experimental project.
Built for learning and exploration.
Maintenance not guaranteed.

---

## Install

From crates.io (when published):

```toml
[dependencies]
leptix = { version = "0.1.0", default-features = false, features = ["csr"] }
leptos = { version = "0.8.15", default-features = false, features = ["csr"] }
```

From a local path:

```toml
[dependencies]
leptix = { path = "../leptix", default-features = false, features = ["csr"] }
leptos = { version = "0.8.15", default-features = false, features = ["csr"] }
```

---

## Usage

```rust
use leptix::Button;
use leptos::{mount::mount_to_body, prelude::*};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main>
            <p>"Count: " {count}</p>
            <Button
                class="my-btn".to_string()
                on_click=Callback::new(move |_event| {
                    set_count.update(|c| *c += 1);
                })
            >
                "Click me"
            </Button>
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
```

---

## Button API

- `class: Option<String>`: optional extra class names.
- `disabled: bool`: disables click behavior and applies disabled styles.
- `on_click: Option<Callback<MouseEvent>>`: click callback.
- `children: Children`: button content.

---

## Example in this repo

- `examples/basic.rs`

Validate it with:

```bash
cargo check --examples --features csr
```

---

## License

MIT. See [`LICENSE.md`](LICENSE.md).
