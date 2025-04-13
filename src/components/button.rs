
use leptos::prelude::*;

#[component]
pub fn Button() -> impl IntoView {

    let on_click = move |_| {
        leptos::logging::log!("Button clicked!");
        leptos::task::spawn_local(async move {
            crate::api::connection_pool_test::api_test().await;
            crate::components::connection_pool_test::components_test().await;
            crate::oncelock::connection_pool_test::oncelock_test().await;
            crate::pages::connection_pool_test::pages_test().await;
        });
    };

    view! {
        <button on:click=on_click>"Test Connection Pool"</button>
    }
}
