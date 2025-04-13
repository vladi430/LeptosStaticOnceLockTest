#[leptos::server(OnceLockConnectionPoolTest, "/cpt2")]
pub async fn oncelock_test() -> Result<(), leptos::prelude::ServerFnError> {
    use crate::oncelock::connection_pool::CONNECTION_POOL;

    let pool = CONNECTION_POOL.get();
    leptos::logging::debug_warn!("[oncelock]: {:?}", pool);
    Ok(())
}