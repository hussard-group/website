mod components;
mod pages;

use components::Layout;
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use pages::{Home, LegalNotice, PrivacyPolicy, TermsOfService};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <main class="legal-page"><div class="container"><h1>"Page not found"</h1></div></main> }>
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("") view=Home/>
                    <Route path=path!("legal-notice") view=LegalNotice/>
                    <Route path=path!("privacy") view=PrivacyPolicy/>
                    <Route path=path!("terms") view=TermsOfService/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
