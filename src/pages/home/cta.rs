use crate::components::ScrollReveal;
use leptos::prelude::*;

#[component]
pub fn Cta() -> impl IntoView {
    view! {
        <section class="cta">
            <div class="container">
                <ScrollReveal>
                    <h2 class="cta-title">
                        "What sets humans apart from animals is the ability to adapt and to learn rapidly, iteratively."
                    </h2>
                </ScrollReveal>
                <ScrollReveal delay_ms=150>
                    <a class="cta-button" href="mailto:hussard.group@proton.me">
                        "Which species are you?"
                    </a>
                </ScrollReveal>
            </div>
        </section>
    }
}
