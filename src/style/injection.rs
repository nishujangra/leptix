pub fn inject_style_once(id: &'static str, css: &'static str) {
    use leptos::prelude::document;

    if document().get_element_by_id(id).is_none() {
        let style = document().create_element("style").unwrap();
        style.set_id(id);
        style.set_text_content(Some(css));
        document().head().unwrap().append_child(&style).unwrap();
    }
}
