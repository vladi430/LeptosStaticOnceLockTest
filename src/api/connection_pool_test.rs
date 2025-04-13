#[leptos::server(ApiConnectionPoolTest, "/cpt0")]
pub async fn api_test() -> Result<(), leptos::prelude::ServerFnError> {
    use crate::oncelock::connection_pool::CONNECTION_POOL;

    let pool = CONNECTION_POOL.get();
    leptos::logging::debug_warn!("[api]: {:?}", pool);
    Ok(())
}