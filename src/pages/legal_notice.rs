use leptos::prelude::*;

#[component]
pub fn LegalNotice() -> impl IntoView {
    view! {
        <main class="legal-page">
            <div class="container">
                <h1>"Legal Notice"</h1>
                <section>
                    <h2>"Publisher"</h2>
                    <p>"This website is published by Hussard Group."</p>
                    <p>"Headquarters: Paris, France"</p>
                    <p>"Contact: hussard.group@proton.me"</p>
                </section>
                <section>
                    <h2>"Publication Director"</h2>
                    <p>"Hussard Group"</p>
                </section>
                <section>
                    <h2>"Hosting"</h2>
                    <p>"This site is hosted by GitHub Pages (GitHub, Inc.)."</p>
                    <p>"88 Colin P. Kelly Jr. Street, San Francisco, CA 94107, United States"</p>
                </section>
                <section>
                    <h2>"Intellectual Property"</h2>
                    <p>"All content on this site, including but not limited to text, graphics, logos, images, and software, is the property of Hussard Group and is protected by French and international intellectual property laws. Any reproduction, distribution, or use without prior written permission is strictly prohibited."</p>
                </section>
            </div>
        </main>
    }
}
