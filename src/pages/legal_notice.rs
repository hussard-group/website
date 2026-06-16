use crate::components::{LegalPage, LegalSection};
use leptos::prelude::*;

#[component]
pub fn LegalNotice() -> impl IntoView {
    view! {
        <LegalPage title="Legal Notice">
            <LegalSection
                title="Publisher"
                content="This website is published by Hussard Group. Headquarters: Paris, France. Contact: hussard.group@proton.me"
            />
            <LegalSection
                title="Publication Director"
                content="Hussard Group"
            />
            <LegalSection
                title="Hosting"
                content="This site is hosted by GitHub Pages (GitHub, Inc.). 88 Colin P. Kelly Jr. Street, San Francisco, CA 94107, United States"
            />
            <LegalSection
                title="Intellectual Property"
                content="All content on this site, including but not limited to text, graphics, logos, images, and software, is the property of Hussard Group and is protected by French and international intellectual property laws. Any reproduction, distribution, or use without prior written permission is strictly prohibited."
            />
        </LegalPage>
    }
}
