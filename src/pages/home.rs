use crate::components::{ImagePlaceholder, Navbar, ScrollReveal};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar/>

        // ===== HERO =====
        <section class="hero">
            <div class="hero-content">
                <span class="hero-brand">"Hussard"</span>
                <h1 class="hero-title">"Ce qui n'existe pas encore."</h1>
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
                        "La plupart des entreprises pensent en trimestres. Nous pensons en "
                        <em>"decennies"</em>
                        ". Nous ne cherchons pas l'optimisation. Nous cherchons "
                        <em>"la rupture"</em>
                        "."
                    </h2>
                </ScrollReveal>
            </div>
        </section>

        // ===== SYNAPTIC =====
        <section class="synaptic">
            <div class="container">
                <div class="synaptic-grid">
                    <ScrollReveal>
                        <div class="synaptic-text">
                            <span class="synaptic-label">"01"</span>
                            <h2 class="synaptic-name">"Synaptic"</h2>
                            <p class="synaptic-desc">
                                "Intelligence artificielle. La premiere entreprise du groupe. Construite pour repousser les limites de ce que l'IA peut accomplir."
                            </p>
                        </div>
                    </ScrollReveal>
                    <ScrollReveal delay_ms=150>
                        <ImagePlaceholder aspect="4 / 3" class="synaptic-img"/>
                    </ScrollReveal>
                </div>
            </div>
        </section>

        // ===== SHOWCASE =====
        <section class="showcase">
            <div class="showcase-inner">
                <ScrollReveal>
                    <ImagePlaceholder aspect="16 / 9" class="showcase-img"/>
                </ScrollReveal>
            </div>
        </section>

        // ===== VALEURS =====
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
                                "Nous ne visons pas le progres incremental. Nous visons les sauts fondamentaux. Chaque entreprise du groupe porte une ambition demesuree."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=200>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"02"</span>
                            <h4 class="pillar-title">"Independance"</h4>
                            <p class="pillar-desc">
                                "Nous ne dependons de personne. Pas de plateforme, pas de stack imposé. Chaque entreprise maitrise sa destinee technique de bout en bout."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=300>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"03"</span>
                            <h4 class="pillar-title">"Rigueur"</h4>
                            <p class="pillar-desc">
                                "Ambition sans execution n'est que reve. Nous codons en Rust, livrons en WebAssembly, optimisons chaque octet. La perfection est exigee."
                            </p>
                        </div>
                    </ScrollReveal>
                </div>
            </div>
        </section>

        // ===== PHILOSOPHIE =====
        <section class="philosophy">
            <div class="container">
                <ScrollReveal>
                    <h3 class="section-label">"Notre methode"</h3>
                </ScrollReveal>

                <ScrollReveal>
                    <div class="philosophy-grid">
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"Rust natif"</span>
                            <span class="philosophy-role">"Memoire sure, concurrence sans peur, performance native"</span>
                        </div>
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"WebAssembly"</span>
                            <span class="philosophy-role">"Le futur du web, execute a la vitesse du metal"</span>
                        </div>
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"Fine-grained"</span>
                            <span class="philosophy-role">"Reactivite chirurgicale, zero Virtual DOM"</span>
                        </div>
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"LTO"</span>
                            <span class="philosophy-role">"Optimisation totale, rien ne gaspille un cycle"</span>
                        </div>
                    </div>
                </ScrollReveal>
            </div>
        </section>

        // ===== CTA =====
        <section class="cta">
            <div class="container">
                <ScrollReveal>
                    <h2 class="cta-title">"L'histoire se souvient de ceux qui construisent."</h2>
                </ScrollReveal>
                <ScrollReveal delay_ms=150>
                    <p class="cta-body">
                        "Si vous partagez cette ambition, nous devrions parler."
                    </p>
                </ScrollReveal>
            </div>
        </section>

        // ===== FOOTER =====
        <footer class="footer">
            <div class="container">
                <span class="footer-brand">"Hussard"</span>
                <span class="footer-copy">"© 2026"</span>
            </div>
        </footer>
    }
}
