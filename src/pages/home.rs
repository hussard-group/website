use crate::components::{Navbar, ScrollReveal};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar/>

        // ===== HERO =====
        <section class="hero">
            <div class="hero-content">
                <h1 class="hero-title">"What does not exist yet."</h1>
            </div>
        </section>

        // ===== MANIFESTO =====
        <section class="manifesto">
            <div class="container">
                <ScrollReveal>
                    <h2 class="manifesto-text">
                        "Most companies think in quarters. We think in "
                        <em>"decades"</em>
                        ". We do not seek optimization. We seek "
                        <em>"the breakthrough"</em>
                        "."
                    </h2>
                </ScrollReveal>
            </div>
        </section>

        // ===== SYNAPTIC =====
        <section class="synaptic">
            <div class="container">
                <ScrollReveal>
                    <div class="synaptic-header">
                        <h2 class="synaptic-name">"Synaptic"</h2>
                    </div>
                </ScrollReveal>
                <ScrollReveal delay_ms=150>
                    <p class="synaptic-desc">
                        "The first company in the group. Pushing the boundaries of what AI can accomplish."
                    </p>
                </ScrollReveal>
            </div>
            <ScrollReveal delay_ms=250>
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
                    <a class="cta-button" href="mailto:hello@hussard.group">
                        "Which species are you?"
                    </a>
                </ScrollReveal>
            </div>
        </section>

        // ===== FOOTER =====
        <footer class="footer">
            <div class="container">
                <span class="footer-brand">"Hussard"</span>
                <span class="footer-copy">"2026"</span>
            </div>
        </footer>
    }
}
