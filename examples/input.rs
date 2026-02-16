use leptix::Input;
use leptos::{mount::mount_to_body, prelude::*};

const DEMO_CSS: &str = r#"
.demo-root {
  padding: 40px;
  font-family: ui-sans-serif, system-ui, sans-serif;
  max-width: 1100px;
  margin: 0 auto;
  color: #0f172a;
}

.demo-subtitle {
  color: #475569;
  margin-top: 0.25rem;
  margin-bottom: 1.5rem;
}

.demo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 1rem;
}

.demo-card {
  border: 1px solid #e2e8f0;
  border-radius: 0.75rem;
  padding: 1rem;
  background: #ffffff;
}

.demo-card h2 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  margin-bottom: 0.85rem;
}

.field:last-child {
  margin-bottom: 0;
}

.field label {
  font-size: 0.8rem;
  font-weight: 600;
  color: #334155;
}

.value {
  margin: 0;
  font-size: 0.75rem;
  color: #64748b;
  word-break: break-all;
}

.leptix-input.is-compact {
  min-height: 2rem;
  padding: 0.35rem 0.6rem;
  font-size: 0.8rem;
}

.leptix-input.is-success-focus {
  --leptix-input-border-focus: #16a34a;
  --leptix-input-ring: rgba(22, 163, 74, 0.2);
}
"#;

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (search, set_search) = signal(String::new());
    let (url, set_url) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (compact, set_compact) = signal(String::new());

    let (age, set_age) = signal("18".to_string());
    let (range_value, set_range_value) = signal("30".to_string());
    let (date_value, set_date_value) = signal(String::new());
    let (time_value, set_time_value) = signal(String::new());
    let (datetime_value, set_datetime_value) = signal(String::new());
    let (month_value, set_month_value) = signal(String::new());
    let (week_value, set_week_value) = signal(String::new());
    let (color_value, set_color_value) = signal("#2563eb".to_string());

    view! {
        <main class="demo-root">
            <style>{DEMO_CSS}</style>

            <h1>"Leptix Input Showcase"</h1>
            <p class="demo-subtitle">
                "Examples for all practical input types supported by the current `Input` component."
            </p>

            <div class="demo-grid">
                <section class="demo-card">
                    <h2>"Core Props"</h2>

                    <div class="field">
                        <label>"Default text (model + placeholder)"</label>
                        <Input
                            placeholder="Enter your name"
                            model=(name.into(), set_name)
                        />
                        <p class="value">"Value: " {move || name.get()}</p>
                    </div>

                    <div class="field">
                        <label>"No model (uncontrolled)"</label>
                        <Input placeholder="This input has no model" />
                    </div>

                    <div class="field">
                        <label>"Custom class"</label>
                        <Input
                            class="is-compact is-success-focus".to_string()
                            placeholder="Compact + custom focus ring"
                            model=(compact.into(), set_compact)
                        />
                        <p class="value">"Value: " {move || compact.get()}</p>
                    </div>
                </section>

                <section class="demo-card">
                    <h2>"Text-like Types"</h2>

                    <div class="field">
                        <label>"Email"</label>
                        <Input r#type="email" placeholder="name@example.com" model=(email.into(), set_email) />
                        <p class="value">"Value: " {move || email.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Password"</label>
                        <Input r#type="password" placeholder="********" model=(password.into(), set_password) />
                        <p class="value">"Length: " {move || password.get().len()}</p>
                    </div>

                    <div class="field">
                        <label>"Search"</label>
                        <Input r#type="search" placeholder="Search..." model=(search.into(), set_search) />
                        <p class="value">"Value: " {move || search.get()}</p>
                    </div>

                    <div class="field">
                        <label>"URL"</label>
                        <Input r#type="url" placeholder="https://example.com" model=(url.into(), set_url) />
                        <p class="value">"Value: " {move || url.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Telephone"</label>
                        <Input r#type="tel" placeholder="+1 555 123 4567" model=(phone.into(), set_phone) />
                        <p class="value">"Value: " {move || phone.get()}</p>
                    </div>
                </section>

                <section class="demo-card">
                    <h2>"Numeric + Date/Time + Color"</h2>

                    <div class="field">
                        <label>"Number"</label>
                        <Input r#type="number" model=(age.into(), set_age) />
                        <p class="value">"Value: " {move || age.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Range"</label>
                        <Input r#type="range" model=(range_value.into(), set_range_value) />
                        <p class="value">"Value: " {move || range_value.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Date"</label>
                        <Input r#type="date" model=(date_value.into(), set_date_value) />
                        <p class="value">"Value: " {move || date_value.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Time"</label>
                        <Input r#type="time" model=(time_value.into(), set_time_value) />
                        <p class="value">"Value: " {move || time_value.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Datetime Local"</label>
                        <Input r#type="datetime-local" model=(datetime_value.into(), set_datetime_value) />
                        <p class="value">"Value: " {move || datetime_value.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Month"</label>
                        <Input r#type="month" model=(month_value.into(), set_month_value) />
                        <p class="value">"Value: " {move || month_value.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Week"</label>
                        <Input r#type="week" model=(week_value.into(), set_week_value) />
                        <p class="value">"Value: " {move || week_value.get()}</p>
                    </div>

                    <div class="field">
                        <label>"Color"</label>
                        <Input r#type="color" model=(color_value.into(), set_color_value) />
                        <p class="value">"Value: " {move || color_value.get()}</p>
                    </div>
                </section>
            </div>
        </main>
    }
}
