use crate::components::ScrollReveal;
use leptos::prelude::*;

#[component]
pub fn Manifesto() -> impl IntoView {
    view! {
        <section class="manifesto">
            <div class="container">
                <ScrollReveal>
                    <h2 class="manifesto-text">
                        "Most stand in awe of human potential. We stand to act. Not merely to witness, but to build what "
                        <em>"outlasts our time"</em>
                        "."
                    </h2>
                </ScrollReveal>
            </div>
        </section>
    }
}
