// Logging tools
use simplelog::*;

/// Sets up logging
/// Does nothing apart from setting up default logging
pub fn setup_logging() -> anyhow::Result<()> {
    // Initalize the logging tools
    Ok(CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        ]
    )?)
}

/// Trait to create anyhow errors
pub trait AnyhowResult<T,E> {
    fn anyhow(self) -> anyhow::Result<T,anyhow::Error>;
}

impl<T,E: std::fmt::Debug> AnyhowResult<T,E> for Result<T,E> {
    /// We can cast a result to an anyhow result
    /// by returning the anyhow result
    fn anyhow(self) -> anyhow::Result<T,anyhow::Error> {
        match self {
            Ok(val) => Ok(val),
            Err(e) => anyhow::bail!(format!("{:?}", e))
        }
    }
}



