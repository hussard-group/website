use crate::components::ScrollReveal;
use leptos::prelude::*;

#[component]
pub fn Values() -> impl IntoView {
    view! {
        <section class="values">
            <div class="container">
                <ScrollReveal>
                    <div class="stats">
                        <div class="stat">
                            <div class="stat-value">"Audacity"</div>
                        </div>
                        <div class="stat-divider"></div>
                        <div class="stat">
                            <div class="stat-value">"Independence"</div>
                        </div>
                        <div class="stat-divider"></div>
                        <div class="stat">
                            <div class="stat-value">"Rigor"</div>
                        </div>
                    </div>
                </ScrollReveal>
            </div>
        </section>
    }
}
