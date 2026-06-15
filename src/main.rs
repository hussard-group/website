mod components;
mod pages;

use pages::Home;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Home);
}
