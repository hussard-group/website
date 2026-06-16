use leptos::prelude::*;

#[component]
pub fn LegalPage(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <main class="legal-page">
            <div class="container">
                <h1>{title}</h1>
                {children()}
            </div>
        </main>
    }
}

#[component]
pub fn LegalSection(
    #[prop(into)] title: String,
    #[prop(into)] content: String,
) -> impl IntoView {
    view! {
        <section>
            <h2>{title}</h2>
            <p>{content}</p>
        </section>
    }
}

#[component]
pub fn LegalUpdated(
    #[prop(into)] date: String,
) -> impl IntoView {
    view! {
        <p class="legal-updated">"Last updated: "{date}</p>
    }
}
