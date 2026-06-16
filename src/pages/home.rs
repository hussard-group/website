use crate::components::ScrollReveal;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        // ===== HERO =====
        <section class="hero">
            <div class="hero-content">
                <h1 class="hero-title">"We are not the strongest. We are the ones who change."</h1>
            </div>
            <div class="hero-arrow">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M7 13l5 5 5-5"/>
                    <path d="M12 18V6"/>
                </svg>
            </div>
        </section>

        // ===== MANIFESTO =====
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

        // ===== SYNAPTIC =====
        <section class="synaptic">
            <ScrollReveal>
                <div class="synaptic-header-wrap">
                    <div class="synaptic-header">
                        <h2 class="synaptic-name">"Synaptic"</h2>
                        <p class="synaptic-desc">
                            "The first company in the group. Pushing the boundaries of what AI can accomplish."
                        </p>
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

        // ===== VALUES =====
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

        // ===== CTA =====
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
