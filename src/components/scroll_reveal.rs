use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverInit};

#[component]
pub fn ScrollReveal(
    #[prop(optional)] delay_ms: i32,
    #[prop(optional)] class: String,
    children: Children,
) -> impl IntoView {
    let node_ref = NodeRef::new();

    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el_html = web_sys::Element::from(el);
            let el_clone = el_html.clone();

            let callback = Closure::wrap(Box::new(move |entries: Vec<JsValue>| {
                if let Some(entry) = entries.first() {
                    let entry: web_sys::IntersectionObserverEntry = entry.clone().unchecked_into();
                    if entry.is_intersecting() {
                        el_clone.class_list().add_1("visible").unwrap();
                    }
                }
            }) as Box<dyn FnMut(Vec<JsValue>)>);

            let options = IntersectionObserverInit::new();
            options.set_threshold(&JsValue::from_f64(0.15));

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap();

            observer.observe(&el_html);
            callback.forget();
        }
    });

    let style = if delay_ms > 0 {
        format!("transition-delay: {}ms;", delay_ms)
    } else {
        String::new()
    };

    view! {
        <div node_ref=node_ref class=format!("reveal {}", class) style=style>
            {children()}
        </div>
    }
}
