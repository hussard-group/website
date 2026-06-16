use crate::components::{LegalPage, LegalSection, LegalUpdated};
use leptos::prelude::*;

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <LegalPage title="Privacy Policy">
            <LegalUpdated date="June 16, 2026"/>
            <LegalSection
                title="1. Data Controller"
                content="The data controller is Hussard Group, contactable at hussard.group@proton.me."
            />
            <LegalSection
                title="2. Data We Collect"
                content="We do not collect personal data automatically when you browse this site. We only process personal data that you voluntarily provide to us when you contact us by email."
            />
            <LegalSection
                title="3. Purpose of Processing"
                content="Your data is used solely to respond to your contact requests and to maintain our professional relationship."
            />
            <LegalSection
                title="4. Legal Basis"
                content="The processing is based on your consent, expressed by contacting us, and on our legitimate interest in responding to your requests."
            />
            <LegalSection
                title="5. Data Retention"
                content="We retain your data for the duration necessary to process your request, and for a maximum of three years from our last contact, unless otherwise required by law."
            />
            <LegalSection
                title="6. Your Rights"
                content="Under the GDPR, you have the right to access, rectify, erase, restrict processing, and port your data. You also have the right to object to processing and to lodge a complaint with the CNIL (French data protection authority)."
            />
            <LegalSection
                title="7. Cookies"
                content="This site does not use cookies or similar tracking technologies."
            />
            <LegalSection
                title="8. Third Parties"
                content="We do not share your data with third parties. Your emails are processed through ProtonMail, which ensures end-to-end encryption."
            />
        </LegalPage>
    }
}
