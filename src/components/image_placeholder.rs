use leptos::prelude::*;

#[component]
pub fn ImagePlaceholder(
    #[prop(default = "16 / 9")]
    aspect: &'static str,
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div
            class=format!("img-placeholder {}", class)
            style=format!("aspect-ratio: {};", aspect)
        >
            <span class="img-placeholder-label">"Image"</span>
        </div>
    }
}
