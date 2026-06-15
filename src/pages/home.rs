use crate::components::{ImagePlaceholder, Navbar, ScrollReveal};
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
                    "Holding de societes technologiques."
                </p>
            </div>
            <ScrollReveal>
                <div class="hero-media">
                    <ImagePlaceholder aspect="21 / 9" class="hero-img"/>
                </div>
            </ScrollReveal>
            <div class="scroll-hint">
                <span class="scroll-line"></span>
            </div>
        </section>

        // ===== MANIFESTO =====
        <section class="manifesto">
            <div class="container">
                <ScrollReveal>
                    <h2 class="manifesto-text">
                        "Nous ne consultons pas. Nous "
                        <em>"construisons"</em>
                        ". Chaque entreprise du groupe est autonome, maitrise sa stack et poursuit une mission claire. Audace, independance et rigueur guident chaque projet du premier commit au premier client."
                    </h2>
                </ScrollReveal>
            </div>
        </section>

        // ===== SYNAPTIC (Premiere entreprise) =====
        <section class="synaptic">
            <div class="container">
                <div class="synaptic-grid">
                    <ScrollReveal>
                        <div class="synaptic-text">
                            <span class="synaptic-label">"01 — Synaptic"</span>
                            <h2 class="synaptic-name">"Synaptic"</h2>
                            <p class="synaptic-desc">
                                "Intelligence artificielle. Notre premiere entreprise, dediee aux solutions IA performantes et souveraines."
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
                                "Nous repoussons les limites. Chaque entreprise est une opportunite d'explorer l'inconnu et de creer ce qui n'existe pas encore."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=200>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"02"</span>
                            <h4 class="pillar-title">"Independance"</h4>
                            <p class="pillar-desc">
                                "Pas de dependance aux stacks legacy. Chaque entreprise choisit les meilleures technologies et les maitrise de bout en bout."
                            </p>
                        </div>
                    </ScrollReveal>

                    <ScrollReveal delay_ms=300>
                        <div class="pillar pillar--inverse">
                            <span class="pillar-number">"03"</span>
                            <h4 class="pillar-title">"Rigueur"</h4>
                            <p class="pillar-desc">
                                "Typage fort, zero overhead, memoire securisee. Chaque ligne de code est pensee, chaque octet compte."
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
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"Rust natif"</span>
                            <span class="philosophy-role">"Typage fort, zero overhead, memoire sure sans GC"</span>
                        </div>
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"WebAssembly"</span>
                            <span class="philosophy-role">"Bytecode binaire execute a vitesse native"</span>
                        </div>
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
                            <span class="philosophy-name">"Fine-grained"</span>
                            <span class="philosophy-role">"Reactivite ciblee, pas de Virtual DOM"</span>
                        </div>
                        <div class="philosophy-item">
                            <ImagePlaceholder aspect="4 / 3" class="philosophy-img"/>
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
                    <h2 class="cta-title">"Construisons la prochaine entreprise ensemble."</h2>
                </ScrollReveal>
                <ScrollReveal delay_ms=150>
                    <p class="cta-body">
                        "Hussard rassemble des developpeurs passionnes, prets a transformer des idees ambitieuses en entreprises technologiques concretes."
                    </p>
                </ScrollReveal>
            </div>
        </section>

        // ===== FOOTER =====
        <footer class="footer">
            <div class="container">
                <span class="footer-brand">"Hussard"</span>
                <span class="footer-copy">"© 2026 — Holding technologique."</span>
            </div>
        </footer>
    }
}
