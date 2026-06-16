pub mod cta;
pub mod hero;
pub mod manifesto;
pub mod synaptic;
pub mod values;

use leptos::prelude::*;

pub use cta::Cta;
pub use hero::Hero;
pub use manifesto::Manifesto;
pub use synaptic::Synaptic;
pub use values::Values;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Hero/>
        <Manifesto/>
        <Synaptic/>
        <Values/>
        <Cta/>
    }
}
