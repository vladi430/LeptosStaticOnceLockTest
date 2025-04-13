#[leptos::server(ComponentsConnectionPoolTest, "/cpt1")]
pub async fn components_test() -> Result<(), leptos::prelude::ServerFnError> {
    use crate::oncelock::connection_pool::CONNECTION_POOL;

    let pool = CONNECTION_POOL.get();
    leptos::logging::debug_warn!("[components]: {:?}", pool);
    Ok(())
}