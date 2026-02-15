use leptix::Button;
use leptos::{mount::mount_to_body, prelude::*};

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <main style="padding:40px; font-family: sans-serif;">
            <h1>"Leptix Button Demo"</h1>

            <p>"Count: " {count}</p>

            <Button
                on_click=Callback::new(move |_event| {
                    set_count.update(|c| *c += 1);
                })
            >
                "Click me"
            </Button>
        </main>
    }
}
