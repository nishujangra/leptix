use crate::style::inject_style_once;
use crate::styles::BUTTON_CSS;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    // ensure styles exist once
    inject_style_once("leptix-button-style", BUTTON_CSS);

    let class = format!("leptix-btn {}", class.unwrap_or_default());

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
