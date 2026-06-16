use leptos::prelude::*;

#[component]
pub fn TermsOfService() -> impl IntoView {
    view! {
        <main class="legal-page">
            <div class="container">
                <h1>"Terms of Service"</h1>
                <p class="legal-updated">"Last updated: June 16, 2026"</p>

                <section>
                    <h2>"1. Acceptance of Terms"</h2>
                    <p>"By accessing and using this website, you accept and agree to be bound by these Terms of Service. If you do not agree, please do not use this site."</p>
                </section>

                <section>
                    <h2>"2. Use of the Site"</h2>
                    <p>"This site is intended for informational purposes only. You may not use this site for any unlawful purpose or in any way that could damage, disable, or overburden our servers or networks."</p>
                </section>

                <section>
                    <h2>"3. Intellectual Property"</h2>
                    <p>"All content, trademarks, and logos displayed on this site are the property of Hussard Group or their respective owners. Nothing on this site should be construed as granting any license or right to use any intellectual property without our express written permission."</p>
                </section>

                <section>
                    <h2>"4. Disclaimer"</h2>
                    <p>"This site and its content are provided 'as is' without any warranties of any kind, either express or implied. We do not guarantee that the site will be error-free, uninterrupted, or free of viruses."</p>
                </section>

                <section>
                    <h2>"5. Limitation of Liability"</h2>
                    <p>"To the fullest extent permitted by law, Hussard Group shall not be liable for any indirect, incidental, special, consequential, or punitive damages arising from your use of this site."</p>
                </section>

                <section>
                    <h2>"6. Governing Law"</h2>
                    <p>"These terms are governed by the laws of France. Any dispute arising from these terms shall be subject to the exclusive jurisdiction of the courts of Paris, France."</p>
                </section>

                <section>
                    <h2>"7. Changes to Terms"</h2>
                    <p>"We reserve the right to modify these terms at any time. Your continued use of the site after any changes indicates your acceptance of the modified terms."</p>
                </section>

                <section>
                    <h2>"8. Contact"</h2>
                    <p>"For any questions regarding these Terms of Service, please contact us at hussard.group@proton.me."</p>
                </section>
            </div>
        </main>
    }
}
