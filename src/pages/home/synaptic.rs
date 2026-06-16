use crate::components::{ExternalLink, ScrollReveal};
use leptos::prelude::*;

#[component]
pub fn Synaptic() -> impl IntoView {
    view! {
        <section class="synaptic">
            <ScrollReveal>
                <div class="synaptic-header-wrap">
                    <div class="synaptic-header">
                        <h2 class="synaptic-name">"Synaptic"</h2>
                        <div class="synaptic-header-right">
                            <p class="synaptic-desc">
                                "The first company in the group. Pushing the boundaries of what AI can accomplish."
                            </p>
                            <a class="synaptic-link" href="https://sinstry.com" target="_blank" rel="noopener noreferrer">
                                "Visit site"
                                <ExternalLink/>
                            </a>
                        </div>
                    </div>
                </div>
            </ScrollReveal>
            <ScrollReveal delay_ms=150>
                <div class="synaptic-img-wrap">
                    <img
                        src="assets/images/data-center.png"
                        alt="Data center"
                        class="synaptic-img"
                    />
                </div>
            </ScrollReveal>
        </section>
    }
}
