use crate::components::{Navbar, ScrollReveal};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar/>

        // ===== HERO =====
        <section class="hero">
            <div class="hero-content">
                <h1 class="hero-title">"Hussard"</h1>
                <p class="hero-subtitle">
                    "On construit l'avenir en Rust."
                </p>
                <p class="hero-cta">
                    "Groupe de développement haute performance."
                </p>
            </div>
            <div class="scroll-hint">
                <span class="scroll-line"></span>
            </div>
        </section>

        // ===== MANIFESTO =====
        <section class="manifesto">
            <div class="container">
                <ScrollReveal>
                    <h2 class="manifesto-text">
                        "Nous croyons que le logiciel de demain doit être "
                        <em>"rapide"</em>
                        ", "
                        <em>"sûr"</em>
                        " et "
                        <em>"élégant"</em>
                        ". C'est pourquoi nous codons en Rust et livrons du WebAssembly natif."
                    </h2>
                </ScrollReveal>
            </div>
        </section>

        // ===== EXPERTISE =====
        <section class="expertise">
            <div class="container">
                <ScrollReveal>
                    <h3 class="section-label">"Notre expertise"</h3>
                </ScrollReveal>

                <div class="pillars">
                    <ScrollReveal delay_ms=100>
                        <div class="pillar">
                            <span class="pillar-number">"01"</span>
                            <h4 class="pillar-title">"Rust natif"</h4>
                            <p class="pillar-desc">
                                "Typage fort, zéro overhead, mémoire sécurisée sans garbage collector."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=200>
                        <div class="pillar">
                            <span class="pillar-number">"02"</span>
                            <h4 class="pillar-title">"WebAssembly"</h4>
                            <p class="pillar-desc">
                                "Bytecode binaire exécuté à vitesse native dans le navigateur."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=300>
                        <div class="pillar">
                            <span class="pillar-number">"03"</span>
                            <h4 class="pillar-title">"Performance"</h4>
                            <p class="pillar-desc">
                                "Optimisations LTO, link-time, panic=abort. Chaque octet compte."
                            </p>
                        </div>
                    </ScrollReveal>
                </div>
            </div>
        </section>

        // ===== STACK =====
        <section class="stack">
            <div class="container">
                <ScrollReveal>
                    <h3 class="section-label">"Stack technique"</h3>
                </ScrollReveal>

                <ScrollReveal>
                    <div class="stack-grid">
                        <div class="stack-item">
                            <span class="stack-name">"Leptos"</span>
                            <span class="stack-role">"Framework réactif fine-grained"</span>
                        </div>
                        <div class="stack-item">
                            <span class="stack-name">"Trunk"</span>
                            <span class="stack-role">"Build & dev server WASM"</span>
                        </div>
                        <div class="stack-item">
                            <span class="stack-name">"wasm-bindgen"</span>
                            <span class="stack-role">"Interop Rust / JS"</span>
                        </div>
                        <div class="stack-item">
                            <span class="stack-name">"wasm-opt"</span>
                            <span class="stack-role">"Optimisation binaire"</span>
                        </div>
                    </div>
                </ScrollReveal>
            </div>
        </section>

        // ===== CTA =====
        <section class="cta">
            <div class="container">
                <ScrollReveal>
                    <h2 class="cta-title">"Construisons quelque chose d'extraordinaire."</h2>
                </ScrollReveal>
                <ScrollReveal delay_ms=150>
                    <p class="cta-body">
                        "Le groupe Hussard rassemble des développeurs passionnés par l'excellence technique."
                    </p>
                </ScrollReveal>
            </div>
        </section>

        // ===== FOOTER =====
        <footer class="footer">
            <div class="container">
                <span class="footer-brand">"Hussard"</span>
                <span class="footer-copy">"© 2026 — Groupe de développement."</span>
            </div>
        </footer>
    }
}
