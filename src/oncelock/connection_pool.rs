#[cfg(feature = "ssr")]
pub static CONNECTION_POOL: std::sync::OnceLock<CustomConnectionType> = std::sync::OnceLock::new();

#[cfg(feature = "ssr")]
pub type CustomConnectionType = std::sync::Arc<i32>;
