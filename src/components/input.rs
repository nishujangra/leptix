use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::{style::inject_style_once, styles::INPUT_CSS};

/// A flexible input field component supporting multiple input types and reactive binding.
///
/// The `Input` component renders an HTML `<input>` element with built-in styling and support
/// for two-way data binding through reactive signals. It handles various input types including
/// text, email, password, number, date, and more.
///
/// # Props
///
/// * `class` - Optional custom CSS class names to apply to the input
/// * `placeholder` - Optional placeholder text displayed when the input is empty
/// * `r#type` - Optional input type (text, email, password, number, date, etc. - defaults to "text")
/// * `model` - Optional two-way binding using Leptos signals `(Signal<String>, WriteSignal<String>)`
///
/// # Examples
///
/// ```rust
/// use leptix::components::Input;
/// use leptos::prelude::*;
///
/// // Simple text input with placeholder
/// view! {
///     <Input placeholder="Enter your name" />
/// }
///
/// // Two-way binding with signals
/// #[component]
/// fn App() -> impl IntoView {
///     let (value, set_value) = signal(String::new());
///
///     view! {
///         <Input
///             placeholder="Type here..."
///             model=(value, set_value)
///         />
///         <p>"You typed: " {move || value.get()}</p>
///     }
/// }
///
/// // Email input with custom styling
/// view! {
///     <Input
///         r#type="email"
///         placeholder="your.email@example.com"
///         class="custom-email-input".to_string()
///     />
/// }
///
/// // Number input with model binding
/// view! {
///     <Input
///         r#type="number"
///         placeholder="Enter a number"
///         model=(number_signal, set_number_signal)
///     />
/// }
/// ```
///
/// # Supported Input Types
///
/// - `text` - Plain text input (default)
/// - `email` - Email validation input
/// - `password` - Password input with hidden characters
/// - `number` - Numeric input with spinner controls
/// - `date` - Date picker input
/// - `time` - Time picker input
/// - `datetime-local` - Date and time picker
/// - `month` - Month picker
/// - `week` - Week picker
/// - `color` - Color picker
/// - `url` - URL validation input
/// - `tel` - Telephone input
/// - `search` - Search input
/// - `range` - Range slider
///
/// # Styling
///
/// The component applies the `leptix-input` class automatically and can be customized via:
/// - Custom CSS classes passed through the `class` prop
/// - CSS custom properties for theme colors and sizes
///
/// # Model Binding
///
/// When a `model` is provided, the input maintains two-way synchronization with the signal:
/// - The input displays the current signal value
/// - User input updates the signal automatically
/// - If no model is provided, the input is uncontrolled
#[component]
pub fn Input(
    /// Optional custom CSS classes to apply to the input
    #[prop(optional)]
    class: Option<String>,
    /// Optional placeholder text shown when input is empty
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Optional input type (text, email, number, date, etc. - defaults to "text")
    #[prop(optional, into)]
    r#type: Option<String>,
    /// Optional two-way binding with reactive signal (value, setter)
    #[prop(optional)]
    model: Option<(Signal<String>, WriteSignal<String>)>,
) -> impl IntoView {
    inject_style_once("leptix-input-style", INPUT_CSS);

    let class = format!("leptix-input {}", class.unwrap_or_default());
    let input_type = r#type.unwrap_or_else(|| "text".to_string());

    view! {
        <input
            class=class
            type=input_type
            placeholder=placeholder.unwrap_or_default()
            prop:value=move || model.as_ref().map(|m| m.0.get()).unwrap_or_default()

            on:input=move |ev| {
                if let Some((_, set)) = model {
                    set.set(event_target_value(&ev));
                }
            }
        />
    }
}
