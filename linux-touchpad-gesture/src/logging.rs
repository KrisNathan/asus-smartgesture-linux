use std::env;
use std::sync::OnceLock;

pub(crate) fn debug_enabled() -> bool {
    static DEBUG_ENABLED: OnceLock<bool> = OnceLock::new();

    *DEBUG_ENABLED.get_or_init(|| {
        env::var("DEBUG")
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(false)
    })
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if $crate::logging::debug_enabled() {
            println!($($arg)*);
        }
    };
}
