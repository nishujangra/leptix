use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::{style::inject_style_once, styles::INPUT_CSS};

#[component]
pub fn Input(
    #[prop(optional)] class: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] r#type: Option<String>,
    #[prop(optional)] model: Option<(Signal<String>, WriteSignal<String>)>,
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
