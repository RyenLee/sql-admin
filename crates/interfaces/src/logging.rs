use crate::config::AppConfig;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};
use tracing_appender::{non_blocking, rolling};

pub struct LogGuard {
    _file_guard: Option<non_blocking::WorkerGuard>,
}

pub fn init_logging(config: &AppConfig) -> LogGuard {
    let filter_str = std::env::var("RUST_LOG").unwrap_or_else(|_| config.log_level.clone());

    if !config.log_dir.is_empty() {
        let file_appender = rolling::daily(&config.log_dir, "sql-admin.log");
        let (non_blocking_file, file_guard) = non_blocking(file_appender);

        let file_layer = fmt::layer()
            .json()
            .with_writer(non_blocking_file)
            .with_filter(EnvFilter::new(&filter_str));

        let console_layer = fmt::layer()
            .json()
            .with_writer(std::io::stdout)
            .with_filter(EnvFilter::new(&filter_str));

        Registry::default()
            .with(file_layer)
            .with(console_layer)
            .init();

        LogGuard {
            _file_guard: Some(file_guard),
        }
    } else {
        let console_layer = fmt::layer()
            .json()
            .with_writer(std::io::stdout)
            .with_filter(EnvFilter::new(&filter_str));

        Registry::default().with(console_layer).init();

        LogGuard { _file_guard: None }
    }
}