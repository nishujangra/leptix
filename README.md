# leptix

Simple UI components for Leptos.

Currently available:
- `Button`
- `Input`

## Compatibility

- `leptix` uses `leptos = 0.8.15`.
- Your app should also use `leptos = 0.8.15` to avoid mixed-version trait errors.
- Leptos `0.8.15` requires `rustc >= 1.88`.

---

## Status

Somehow alive. May die at any moment. No guarantees.

---

## Install

From crates.io (when published):

```toml
[dependencies]
leptix = { version = "0.1.1", default-features = false, features = ["csr"] }
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
use leptix::{Button, Input};
use leptos::{mount::mount_to_body, prelude::*};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal(String::new());

    view! {
        <main>
            <Input
                placeholder="Your name"
                model=(name.into(), set_name)
            />
            <p>"Hello, " {move || name.get()}</p>

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

## Input API

- `class: Option<String>`: optional extra class names.
- `placeholder: Option<String>`: placeholder text (supports `#[prop(into)]`).
- `r#type: Option<String>`: input type, defaults to `"text"`.
- `model: Option<(Signal<String>, WriteSignal<String>)>`: optional two-way string model.

Example type usage:

```rust
view! {
    <Input r#type="email" placeholder="name@example.com" />
}
```

---

## Examples in this repo

- `examples/basic.rs` (button showcase)
- `examples/input.rs` (input showcase)

Validate it with:

```bash
cargo check --examples --features csr
```

---

## License

MIT. See [`LICENSE.md`](LICENSE.md).
