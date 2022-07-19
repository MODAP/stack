// Logging tools
use simplelog::*;

/// Sets up logging
/// Does nothing apart from setting up default logging
pub fn setup_logging() -> anyhow::Result<()> {
    // Initalize the logging tools
    Ok(CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])?)
}

pub trait ResultExt<T, E> {
    fn log_error(self, msg: &'static str) -> Option<T>;
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error>;
}

impl<T, E: std::fmt::Debug> ResultExt<T, E> for Result<T, E> {
    fn log_error(self, msg: &'static str) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(e) => {
                log::error!("{} {:?}", msg, e);
                None
            }
        }
    }
    fn anyhow(self) -> anyhow::Result<T, anyhow::Error> {
        match self {
            Ok(val) => Ok(val),
            Err(e) => anyhow::bail!(format!("{:?}", e)),
        }
    }
}
