use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-main">
                    <nav class="footer-nav">
                        <a href="mailto:hussard.group@proton.me">"Contact"</a>
                        <A href="/legal-notice">"Legal Notice"</A>
                        <A href="/privacy">"Privacy Policy"</A>
                        <A href="/terms">"Terms of Service"</A>
                    </nav>
                </div>
                <div class="footer-bottom">
                    <span class="footer-copy">"2026 Hussard Group. All rights reserved."</span>
                </div>
            </div>
        </footer>
    }
}
