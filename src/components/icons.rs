use leptos::prelude::*;

#[component]
pub fn ArrowDown() -> impl IntoView {
    view! {
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M7 13l5 5 5-5"/>
            <path d="M12 18V6"/>
        </svg>
    }
}

#[component]
pub fn ExternalLink() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M7 17L17 7"/>
            <path d="M7 7h10v10"/>
        </svg>
    }
}
