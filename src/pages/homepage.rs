use leptos::prelude::*;

use crate::components::button::Button;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Button />
    }
}