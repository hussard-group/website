use crate::components::ArrowDown;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-content">
                <h1 class="hero-title">"We are not the strongest. We are the ones who change."</h1>
            </div>
            <div class="hero-arrow">
                <ArrowDown/>
            </div>
        </section>
    }
}
