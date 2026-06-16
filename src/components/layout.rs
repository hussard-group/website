use crate::components::{Footer, Navbar};
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <Navbar/>
        <Outlet/>
        <Footer/>
    }
}
