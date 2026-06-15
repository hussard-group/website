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
                    "Groupe de développement au service des entreprises"
                </p>
                <p class="hero-cta">
                    "fortes d'innovation."
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
                        "Nous croyons que les entreprises qui osent innover méritent des partenaires "
                        <em>"audacieux"</em>
                        ", "
                        <em>"indépendants"</em>
                        " et "
                        <em>"rigoureux"</em>
                        ". C'est dans cette alliance que se construisent les projets d'exception."
                    </h2>
                </ScrollReveal>
            </div>
        </section>

        // ===== VALEURS (sub-dark) =====
        <section class="values">
            <div class="container">
                <ScrollReveal>
                    <h3 class="section-label">"Nos valeurs"</h3>
                </ScrollReveal>

                <div class="pillars">
                    <ScrollReveal delay_ms=100>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"01"</span>
                            <h4 class="pillar-title">"Audace"</h4>
                            <p class="pillar-desc">
                                "Nous repoussons les limites. Chaque projet est une opportunité d'explorer l'inconnu et de créer ce qui n'existe pas encore."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=200>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"02"</span>
                            <h4 class="pillar-title">"Indépendance"</h4>
                            <p class="pillar-desc">
                                "Pas de dépendance aux stacks legacy. Nous choisissons les meilleures technologies — et les maîtrisons de bout en bout."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=300>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"03"</span>
                            <h4 class="pillar-title">"Rigueur"</h4>
                            <p class="pillar-desc">
                                "Typage fort, zéro overhead, mémoire sécurisée. Chaque ligne de code est pensée, chaque octet compte."
                            </p>
                        </div>
                    </ScrollReveal>
                </div>
            </div>
        </section>

        // ===== PHILOSOPHIE (light) =====
        <section class="philosophy">
            <div class="container">
                <ScrollReveal>
                    <h3 class="section-label">"Notre philosophie"</h3>
                </ScrollReveal>

                <ScrollReveal>
                    <div class="philosophy-grid">
                        <div class="philosophy-item">
                            <span class="philosophy-name">"Rust natif"</span>
                            <span class="philosophy-role">"Typage fort, zéro overhead, mémoire sûre sans GC"</span>
                        </div>
                        <div class="philosophy-item">
                            <span class="philosophy-name">"WebAssembly"</span>
                            <span class="philosophy-role">"Bytecode binaire exécuté à vitesse native"</span>
                        </div>
                        <div class="philosophy-item">
                            <span class="philosophy-name">"Fine-grained"</span>
                            <span class="philosophy-role">"Réactivité ciblée, pas de Virtual DOM"</span>
                        </div>
                        <div class="philosophy-item">
                            <span class="philosophy-name">"LTO + opt-level=z"</span>
                            <span class="philosophy-role">"Link-time optimization, panic=abort, strip"</span>
                        </div>
                    </div>
                </ScrollReveal>
            </div>
        </section>

        // ===== CTA =====
        <section class="cta">
            <div class="container">
                <ScrollReveal>
                    <h2 class="cta-title">"Construisons l'exceptionnel, ensemble."</h2>
                </ScrollReveal>
                <ScrollReveal delay_ms=150>
                    <p class="cta-body">
                        "Le groupe Hussard rassemble des développeurs passionnés, prêts à relever vos défis les plus ambitieux."
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
