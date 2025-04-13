#[leptos::server(PagesConnectionPoolTest, "/cpt3")]
pub async fn pages_test() -> Result<(), leptos::prelude::ServerFnError> {
    use crate::oncelock::connection_pool::CONNECTION_POOL;

    let pool = CONNECTION_POOL.get();
    leptos::logging::debug_warn!("[pages]: {:?}", pool);
    Ok(())
}