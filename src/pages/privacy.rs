use leptos::prelude::*;

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <main class="legal-page">
            <div class="container">
                <h1>"Privacy Policy"</h1>
                <p class="legal-updated">"Last updated: June 16, 2026"</p>

                <section>
                    <h2>"1. Data Controller"</h2>
                    <p>"The data controller is Hussard Group, contactable at hussard.group@proton.me."</p>
                </section>

                <section>
                    <h2>"2. Data We Collect"</h2>
                    <p>"We do not collect personal data automatically when you browse this site. We only process personal data that you voluntarily provide to us when you contact us by email."</p>
                </section>

                <section>
                    <h2>"3. Purpose of Processing"</h2>
                    <p>"Your data is used solely to respond to your contact requests and to maintain our professional relationship."</p>
                </section>

                <section>
                    <h2>"4. Legal Basis"</h2>
                    <p>"The processing is based on your consent, expressed by contacting us, and on our legitimate interest in responding to your requests."</p>
                </section>

                <section>
                    <h2>"5. Data Retention"</h2>
                    <p>"We retain your data for the duration necessary to process your request, and for a maximum of three years from our last contact, unless otherwise required by law."</p>
                </section>

                <section>
                    <h2>"6. Your Rights"</h2>
                    <p>"Under the GDPR, you have the right to access, rectify, erase, restrict processing, and port your data. You also have the right to object to processing and to lodge a complaint with the CNIL (French data protection authority)."</p>
                </section>

                <section>
                    <h2>"7. Cookies"</h2>
                    <p>"This site does not use cookies or similar tracking technologies."</p>
                </section>

                <section>
                    <h2>"8. Third Parties"</h2>
                    <p>"We do not share your data with third parties. Your emails are processed through ProtonMail, which ensures end-to-end encryption."</p>
                </section>
            </div>
        </main>
    }
}
