use crate::config::AppConfig;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};
use tracing_appender::{non_blocking, rolling};

pub struct LogGuard {
    _file_guard: Option<non_blocking::WorkerGuard>,
}

pub fn init_logging(config: &AppConfig) -> LogGuard {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    // 初始化日志
    // 配置环境变量日志
    if !config.environment.is_production() && !config.log_dir.is_empty() {
        // 非生产环境，且日志目录不为空
        // 配置文件日志
        // 配置控制台日志
        let file_appender = rolling::daily(&config.log_dir, "sql-admin.log");
        let (non_blocking_file, file_guard) = non_blocking(file_appender);

        let file_layer = fmt::layer()
            .json()
            .with_writer(non_blocking_file)
            .with_filter(env_filter);

        let console_layer = fmt::layer()
            .json()
            .with_writer(std::io::stdout)
            .with_filter(EnvFilter::new("warn"));

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
            .with_filter(env_filter);

        Registry::default()
            .with(console_layer)
            .init();

        LogGuard { _file_guard: None }
    }
}
