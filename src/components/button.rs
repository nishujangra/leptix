use crate::style::inject_style_once;
use crate::styles::BUTTON_CSS;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
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

#[component]
pub fn Button(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,
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
