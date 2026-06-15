use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Navbar() -> impl IntoView {
    let scrolled = RwSignal::new(false);

    let on_scroll = move || {
        let window = web_sys::window().unwrap();
        let scroll_y = window.scroll_y().unwrap_or(0.0);
        scrolled.set(scroll_y > 40.0);
    };

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(on_scroll) as Box<dyn Fn()>);
        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });

    view! {
        <nav class="navbar" class:scrolled=scrolled>
            <div class="nav-inner"></div>
        </nav>
    }
}
