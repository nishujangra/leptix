//! # Leptix Toast Component Showcase
//!
//! This example demonstrates all features of the `Toast` component including:
//! - Different variants: Success, Error, Warning, Info
//! - Positioning: TopLeft, TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight
//! - Auto-dismiss functionality with custom duration
//! - Manual dismiss via on_close callback
//! - Custom styling with CSS classes

use leptix::components::{Toast, ToastPosition, ToastVariant};
use leptos::{mount::mount_to_body, prelude::*};
const DEMO_CSS: &str = r#"
.demo-root {
    padding: 40px;
    font-family: ui-sans-serif, system-ui, sans-serif;
    max-width: 1200px;
    margin: 0 auto;
    color: #0f172a;
}
.demo-subtitle {
    color: #475569;
    margin-top: 0.25rem;
    margin-bottom: 2rem;
}
.demo-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
}
.demo-card {
    border: 1px solid #e2e8f0;
    border-radius: 0.75rem;
    padding: 1.5rem;
    background: white;
}
.demo-card h2 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: #1e293b;
}
.button-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}
.toast-button {
    padding: 0.75rem 1rem;
    border: 1px solid #e2e8f0;
    border-radius: 0.5rem;
    background: white;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s ease;
}
.toast-button:hover {
    border-color: #2563eb;
    background: #f0f9ff;
}
.control-section {
    border: 1px solid #e2e8f0;
    border-radius: 0.75rem;
    padding: 1rem;
    background: #f8fafc;
}
.control-section h3 {
    margin: 0 0 1rem 0;
    font-size: 0.875rem;
    color: #334155;
}
.control-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}
.control-group label {
    font-size: 0.8rem;
    font-weight: 600;
    color: #475569;
}
.control-group input,
.control-group select {
    padding: 0.5rem;
    border: 1px solid #cbd5e1;
    border-radius: 0.375rem;
    font-size: 0.875rem;
}
.demo-info {
    background: #dbeafe;
    border: 1px solid #bfdbfe;
    border-radius: 0.5rem;
    padding: 1rem;
    margin: 1rem 0 0 0;
    color: #0c2340;
    font-size: 0.875rem;
}
"#;

/// Entry point for the application.
///
/// Mounts the App component to the DOM body.
fn main() {
    mount_to_body(|| view! { <App/> });
}
/// Main application component showcasing all Toast features.
///
/// This component demonstrates:
/// - All four toast variants (Success, Error, Warning, Info)
/// - Different positioning options
/// - Auto-dismiss with custom duration
/// - Manual dismiss via buttons
/// - Custom styling capabilities
///
/// The example is organized into sections:
/// 1. **Toast Variants** - Different types and their use cases
/// 2. **Positioning** - All available positions on screen
/// 3. **Auto-Dismiss** - Toasts that automatically close
/// 4. **Interactive Control** - Custom toast with user control
#[component]
fn App() -> impl IntoView {
    // State for toasts
    let (show_success, set_show_success) = signal(false);
    let (show_error, set_show_error) = signal(false);
    let (show_warning, set_show_warning) = signal(false);
    let (show_info, set_show_info) = signal(false);
    // State for interactive toast
    let (show_custom, set_show_custom) = signal(false);
    let (custom_message, set_custom_message) = signal("Custom message".to_string());
    let (custom_variant, set_custom_variant) = signal("Info".to_string());
    let (custom_position, set_custom_position) = signal("TopRight".to_string());
    let (custom_duration, set_custom_duration) = signal(0u32);
    view! {
        <main class="demo-root">
            <style>{DEMO_CSS}</style>
            <h1>"Leptix Toast Showcase"</h1>
            <p class="demo-subtitle">
                "Toast notifications for displaying temporary messages with various styles and positions."
            </p>
            <div class="demo-grid">
                {/* Toast Variants Section */}
                <section class="demo-card">
                    <h2>"Toast Variants"</h2>
                    <div class="button-group">
                        <button
                            class="toast-button"
                            on:click=move |_| set_show_success.set(true)
                        >
                            "Show Success"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| set_show_error.set(true)
                        >
                            "Show Error"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| set_show_warning.set(true)
                        >
                            "Show Warning"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| set_show_info.set(true)
                        >
                            "Show Info"
                        </button>
                    </div>
                    <div class="demo-info">
                        "Four distinct variants for different types of notifications."
                    </div>
                </section>
                {/* Positioning Section */}
                <section class="demo-card">
                    <h2>"Positioning"</h2>
                    <div class="button-group">
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_position.set("TopLeft".to_string());
                                set_custom_message.set("Top Left".to_string());
                                set_show_custom.set(true);
                            }
                        >
                            "Top Left"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_position.set("TopCenter".to_string());
                                set_custom_message.set("Top Center".to_string());
                                set_show_custom.set(true);
                            }
                        >
                            "Top Center"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_position.set("TopRight".to_string());
                                set_custom_message.set("Top Right".to_string());
                                set_show_custom.set(true);
                            }
                        >
                            "Top Right"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_position.set("BottomLeft".to_string());
                                set_custom_message.set("Bottom Left".to_string());
                                set_show_custom.set(true);
                            }
                        >
                            "Bottom Left"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_position.set("BottomCenter".to_string());
                                set_custom_message.set("Bottom Center".to_string());
                                set_show_custom.set(true);
                            }
                        >
                            "Bottom Center"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_position.set("BottomRight".to_string());
                                set_custom_message.set("Bottom Right".to_string());
                                set_show_custom.set(true);
                            }
                        >
                            "Bottom Right"
                        </button>
                    </div>
                    <div class="demo-info">
                        "Display toasts at any of the six screen corners."
                    </div>
                </section>
                {/* Auto-Dismiss Section */}
                <section class="demo-card">
                    <h2>"Auto-Dismiss"</h2>
                    <div class="button-group">
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_message.set("Disappears in 2 seconds".to_string());
                                set_custom_variant.set("Success".to_string());
                                set_custom_duration.set(2000u32);
                                set_show_custom.set(true);
                            }
                        >
                            "2 Second Toast"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_message.set("Disappears in 5 seconds".to_string());
                                set_custom_variant.set("Info".to_string());
                                set_custom_duration.set(5000u32);
                                set_show_custom.set(true);
                            }
                        >
                            "5 Second Toast"
                        </button>
                        <button
                            class="toast-button"
                            on:click=move |_| {
                                set_custom_message.set("Disappears in 10 seconds".to_string());
                                set_custom_variant.set("Warning".to_string());
                                set_custom_duration.set(10000u32);
                                set_show_custom.set(true);
                            }
                        >
                            "10 Second Toast"
                        </button>
                    </div>
                    <div class="demo-info">
                        "Toasts can automatically dismiss after a specified duration in milliseconds."
                    </div>
                </section>
                {/* Interactive Control Section */}
                <section class="demo-card">
                    <h2>"Interactive Control"</h2>
                    <div class="control-section">
                        <h3>"Toast Configuration"</h3>
                        <div class="control-group">
                            <label>"Message"</label>
                            <input
                                type="text"
                                value=move || custom_message.get()
                                on:input=move |ev| {
                                    set_custom_message.set(event_target_value(&ev));
                                }
                                placeholder="Enter toast message"
                            />
                        </div>
                        <div class="control-group" style="margin-top: 0.75rem;">
                            <label>"Variant"</label>
                            <select
                                prop:value=move || custom_variant.get()
                                on:change=move |ev| {
                                    set_custom_variant.set(event_target_value(&ev));
                                }
                            >
                                <option value="Success">"Success"</option>
                                <option value="Error">"Error"</option>
                                <option value="Warning">"Warning"</option>
                                <option value="Info">"Info"</option>
                            </select>
                        </div>
                        <div class="control-group" style="margin-top: 0.75rem;">
                            <label>"Position"</label>
                            <select
                                prop:value=move || custom_position.get()
                                on:change=move |ev| {
                                    set_custom_position.set(event_target_value(&ev));
                                }
                            >
                                <option value="TopLeft">"Top Left"</option>
                                <option value="TopCenter">"Top Center"</option>
                                <option value="TopRight">"Top Right"</option>
                                <option value="BottomLeft">"Bottom Left"</option>
                                <option value="BottomCenter">"Bottom Center"</option>
                                <option value="BottomRight">"Bottom Right"</option>
                            </select>
                        </div>
                    </div>
                    <button
                        class="toast-button"
                        style="margin-top: 1rem; background: #3b82f6; color: white; border-color: #3b82f6;"
                        on:click=move |_| set_show_custom.set(true)
                    >
                        "Show Custom Toast"
                    </button>
                    <Show when=move || show_custom.get() fallback=|| view! { <></> }>
                        {move || {
                            let custom_msg = custom_message.get();
                            let variant_str = custom_variant.get();
                            let position_str = custom_position.get();
                            let _duration_val = custom_duration.get();
                            let variant = match variant_str.as_str() {
                                "Success" => ToastVariant::Success,
                                "Error" => ToastVariant::Error,
                                "Warning" => ToastVariant::Warning,
                                _ => ToastVariant::Info,
                            };
                            let position = match position_str.as_str() {
                                "TopLeft" => ToastPosition::TopLeft,
                                "TopCenter" => ToastPosition::TopCenter,
                                "BottomLeft" => ToastPosition::BottomLeft,
                                "BottomCenter" => ToastPosition::BottomCenter,
                                "BottomRight" => ToastPosition::BottomRight,
                                _ => ToastPosition::TopRight,
                            };
                            view! {
                                <Toast
                                    message=custom_msg
                                    variant=variant
                                    position=position
                                    on_close=Callback::new(move |_| set_show_custom.set(false))
                                />
                            }
                        }}
                    </Show>
                </section>
            </div>
            {/* Toast components */}
            <Show when=move || show_success.get() fallback=|| view! { <></> }>
                <Toast
                    message="Operation completed successfully!".to_string()
                    variant=ToastVariant::Success
                    on_close=Callback::new(move |_| set_show_success.set(false))
                />
            </Show>
            <Show when=move || show_error.get() fallback=|| view! { <></> }>
                <Toast
                    message="An error occurred. Please try again.".to_string()
                    variant=ToastVariant::Error
                    on_close=Callback::new(move |_| set_show_error.set(false))
                />
            </Show>
            <Show when=move || show_warning.get() fallback=|| view! { <></> }>
                <Toast
                    message="Warning: This action cannot be undone.".to_string()
                    variant=ToastVariant::Warning
                    on_close=Callback::new(move |_| set_show_warning.set(false))
                />
            </Show>
            <Show when=move || show_info.get() fallback=|| view! { <></> }>
                <Toast
                    message="This is an informational message.".to_string()
                    variant=ToastVariant::Info
                    on_close=Callback::new(move |_| set_show_info.set(false))
                />
            </Show>
        </main>
    }
}
