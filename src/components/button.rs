use crate::style::inject_style_once;
use crate::styles::BUTTON_CSS;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Defines the visual variant of a button.
///
/// Each variant has a distinct color and styling to communicate different
/// actions and importance levels to the user.
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    /// Primary variant (blue) - Use for main actions
    Primary,
    /// Secondary variant (gray) - Use for secondary actions
    Secondary,
    /// Danger variant (red) - Use for destructive actions
    Danger,
    /// Outline variant - Transparent with border
    Outline,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "leptix-btn-primary",
            ButtonVariant::Secondary => "leptix-btn-secondary",
            ButtonVariant::Danger => "leptix-btn-danger",
            ButtonVariant::Outline => "leptix-btn-outline",
        }
    }
}

/// A reusable button component with multiple variants and styling options.
///
/// The `Button` component renders an HTML `<button>` element with built-in
/// styling and support for different visual variants. It handles click events
/// and disabled states automatically.
///
/// # Props
///
/// * `class` - Optional custom CSS class names to apply to the button
/// * `disabled` - Whether the button is disabled (default: false)
/// * `variant` - The visual variant of the button (default: Primary)
/// * `on_click` - Optional callback function triggered when the button is clicked
/// * `children` - The content to display inside the button
///
/// # Examples
///
/// ```rust
/// use leptix::components::{Button, ButtonVariant};
///
/// // Primary button (default)
/// view! {
///     <Button on_click=callback>"Click me"</Button>
/// }
///
/// // Danger button
/// view! {
///     <Button variant=ButtonVariant::Danger on_click=delete_callback>"Delete"</Button>
/// }
///
/// // Disabled button
/// view! {
///     <Button disabled=true>"Disabled"</Button>
/// }
///
/// // Custom styling
/// view! {
///     <Button class="my-custom-class".to_string()>"Custom"</Button>
/// }
/// ```
#[component]
pub fn Button(
    /// Optional custom CSS classes to apply to the button
    #[prop(optional)]
    class: Option<String>,
    /// Whether the button is disabled (default: false)
    #[prop(optional)]
    disabled: bool,
    /// The visual variant style (default: Primary)
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Callback function triggered on click events
    #[prop(optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// The content rendered inside the button
    children: Children,
) -> impl IntoView {
    // ensure styles exist once
    inject_style_once("leptix-button-style", BUTTON_CSS);

    let variant_class = variant.unwrap_or(ButtonVariant::Primary).class();
    let class = format!("leptix-btn {} {}", variant_class, class.unwrap_or_default());

    view! {
        <button
            class=class
            disabled=disabled
            on:click=move |event| {
                if !disabled {
                    if let Some(cb) = on_click {
                        cb.run(event);
                    }
                }
            }
        >
            {children()}
        </button>
    }
}
