use crate::style::inject_style_once;
use crate::styles::TOAST_CSS;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Defines the type/variant of a toast notification.
///
/// Each variant represents a different notification type with distinct styling
/// to communicate the nature of the message to the user.
#[derive(Clone, Copy, PartialEq)]
pub enum ToastVariant {
    /// Success variant (green) - Use for successful operations
    Success,
    /// Error variant (red) - Use for errors and failures
    Error,
    /// Warning variant (yellow) - Use for warnings
    Warning,
    /// Info variant (blue) - Use for informational messages
    Info,
}

impl ToastVariant {
    fn class(&self) -> &'static str {
        match self {
            ToastVariant::Success => "leptix-toast-success",
            ToastVariant::Error => "leptix-toast-error",
            ToastVariant::Warning => "leptix-toast-warning",
            ToastVariant::Info => "leptix-toast-info",
        }
    }
}

/// Defines the position of the toast on the screen.
#[derive(Clone, Copy, PartialEq)]
pub enum ToastPosition {
    /// Top-left corner
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
}

impl ToastPosition {
    fn class(&self) -> &'static str {
        match self {
            ToastPosition::TopLeft => "leptix-toast-top-left",
            ToastPosition::TopCenter => "leptix-toast-top-center",
            ToastPosition::TopRight => "leptix-toast-top-right",
            ToastPosition::BottomLeft => "leptix-toast-bottom-left",
            ToastPosition::BottomCenter => "leptix-toast-bottom-center",
            ToastPosition::BottomRight => "leptix-toast-bottom-right",
        }
    }
}

/// A toast notification component for displaying temporary messages.
///
/// The `Toast` component displays a dismissible notification message with different
/// variants (success, error, warning, info) and customizable positioning.
///
/// # Props
///
/// * `message` - The text content to display in the toast
/// * `variant` - The type of notification (default: Info)
/// * `position` - Where to display the toast on the screen (default: TopRight)
/// * `is_open` - Whether the toast is visible (default: true)
/// * `on_close` - Optional callback triggered when the toast is dismissed
/// * `class` - Optional custom CSS classes to apply
/// * `duration` - Optional auto-dismiss duration in milliseconds
///
/// # Examples
///
/// ```rust
/// use leptix::components::{Toast, ToastVariant, ToastPosition};
///
/// // Simple success toast
/// view! {
///     <Toast
///         message="Operation completed successfully"
///         variant=ToastVariant::Success
///     />
/// }
///
/// // Error toast with close callback
/// view! {
///     <Toast
///         message="An error occurred"
///         variant=ToastVariant::Error
///         position=ToastPosition::TopCenter
///         on_close=close_callback
///     />
/// }
///
/// // Info toast with auto-dismiss
/// view! {
///     <Toast
///         message="This is an informational message"
///         variant=ToastVariant::Info
///         duration=Some(3000)
///     />
/// }
/// ```
#[component]
pub fn Toast(
    /// The message text to display in the toast
    message: String,
    /// The variant/type of toast (default: Info)
    #[prop(optional)]
    variant: Option<ToastVariant>,
    /// The position on the screen (default: TopRight)
    #[prop(optional)]
    position: Option<ToastPosition>,
    /// Whether the toast is visible (default: true)
    #[prop(optional)]
    is_open: Option<bool>,
    /// Callback function triggered when the toast is dismissed
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Optional custom CSS classes
    #[prop(optional)]
    class: Option<String>,
    /// Optional duration in milliseconds before auto-dismissing
    #[prop(optional)]
    duration: Option<u32>,
) -> impl IntoView {
    // ensure styles exist once
    inject_style_once("leptix-toast-style", TOAST_CSS);

    let variant_cls = variant.unwrap_or(ToastVariant::Info).class();
    let position_cls = position.unwrap_or(ToastPosition::TopRight).class();
    let is_visible = is_open.unwrap_or(true);
    let toast_class = format!(
        "leptix-toast {} {} {}",
        variant_cls,
        position_cls,
        class.unwrap_or_default()
    );

    // Handle auto-dismiss
    if let Some(duration) = duration {
        let on_close_cb = on_close;
        spawn_local(async move {
            gloo_timers::future::sleep(std::time::Duration::from_millis(duration as u64)).await;
            if let Some(cb) = on_close_cb {
                cb.run(());
            }
        });
    }
    let toast_class_clone = toast_class.clone();
    let message_clone = message.clone();

    view! {
        <Show
            when=move || is_visible
            fallback=|| view! { <></> }
        >
            <div class=toast_class_clone.clone()>
                <div class="leptix-toast-content">{message_clone.clone()}</div>
                <button
                    class="leptix-toast-close"
                    on:click=move |_| {
                        if let Some(cb) = on_close {
                            cb.run(());
                        }
                    }
                >
                    "×"
                </button>
            </div>
        </Show>
    }
}
