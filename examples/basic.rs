use leptix::Button;
use leptos::{mount::mount_to_body, prelude::*};

const DEMO_CSS: &str = r#"
.demo-root {
  padding: 40px;
  font-family: ui-sans-serif, system-ui, sans-serif;
  max-width: 960px;
  margin: 0 auto;
  color: #0f172a;
}

.demo-subtitle {
  color: #475569;
  margin-top: 0.25rem;
}

.demo-stats {
  margin: 1rem 0 1.5rem 0;
  padding: 0.75rem 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 0.75rem;
  background: #f8fafc;
}

.demo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 1rem;
}

.demo-card {
  border: 1px solid #e2e8f0;
  border-radius: 0.75rem;
  padding: 1rem;
  background: white;
}

.demo-card h2 {
  margin: 0 0 0.75rem 0;
  font-size: 1rem;
}

.demo-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.leptix-btn.btn-secondary {
  --leptix-primary: #334155;
  --leptix-primary-hover: #1e293b;
}

.leptix-btn.btn-success {
  --leptix-primary: #16a34a;
  --leptix-primary-hover: #15803d;
}

.leptix-btn.btn-danger {
  --leptix-primary: #dc2626;
  --leptix-primary-hover: #b91c1c;
}

.leptix-btn.btn-outline {
  background: transparent;
  color: #2563eb;
  border: 1px solid #2563eb;
}

.leptix-btn.btn-outline:hover {
  background: #eff6ff;
}

.leptix-btn.btn-ghost {
  background: transparent;
  color: #334155;
}

.leptix-btn.btn-ghost:hover {
  background: #f1f5f9;
}

.leptix-btn.btn-sm {
  padding: 0.35rem 0.75rem;
  font-size: 0.75rem;
}

.leptix-btn.btn-lg {
  padding: 0.75rem 1.25rem;
  font-size: 1rem;
}

.leptix-btn.btn-pill {
  border-radius: 9999px;
}
"#;

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (last_clicked, set_last_clicked) = signal("None".to_string());

    let make_click_cb = move |label: &'static str| {
        let set_count = set_count;
        let set_last_clicked = set_last_clicked;
        Callback::new(move |_event| {
            set_count.update(|c| *c += 1);
            set_last_clicked.set(label.to_string());
        })
    };

    view! {
        <main class="demo-root">
            <style>{DEMO_CSS}</style>

            <h1>"Leptix Button Showcase"</h1>
            <p class="demo-subtitle">"All button types currently possible with the `Button` component."</p>

            <div class="demo-stats">
                <strong>"Total Clicks: "</strong>{count}
                <br />
                <strong>"Last Clicked: "</strong>{last_clicked}
            </div>

            <div class="demo-grid">
                <section class="demo-card">
                    <h2>"Core Behavior"</h2>
                    <div class="demo-row">
                        <Button on_click=make_click_cb("Primary")>"Primary"</Button>
                        <Button disabled=true on_click=make_click_cb("Disabled")>"Disabled"</Button>
                        <Button>"No Callback"</Button>
                    </div>
                </section>

                <section class="demo-card">
                    <h2>"Semantic Colors (via CSS vars)"</h2>
                    <div class="demo-row">
                        <Button class="btn-secondary".to_string() on_click=make_click_cb("Secondary")>"Secondary"</Button>
                        <Button class="btn-success".to_string() on_click=make_click_cb("Success")>"Success"</Button>
                        <Button class="btn-danger".to_string() on_click=make_click_cb("Danger")>"Danger"</Button>
                    </div>
                </section>

                <section class="demo-card">
                    <h2>"Visual Styles (via class)"</h2>
                    <div class="demo-row">
                        <Button class="btn-outline".to_string() on_click=make_click_cb("Outline")>"Outline"</Button>
                        <Button class="btn-ghost".to_string() on_click=make_click_cb("Ghost")>"Ghost"</Button>
                        <Button class="btn-pill".to_string() on_click=make_click_cb("Pill")>"Pill"</Button>
                    </div>
                </section>

                <section class="demo-card">
                    <h2>"Sizes"</h2>
                    <div class="demo-row">
                        <Button class="btn-sm".to_string() on_click=make_click_cb("Small")>"Small"</Button>
                        <Button on_click=make_click_cb("Medium")>"Medium"</Button>
                        <Button class="btn-lg".to_string() on_click=make_click_cb("Large")>"Large"</Button>
                    </div>
                </section>
            </div>
        </main>
    }
}
