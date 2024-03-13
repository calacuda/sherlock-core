use crate::SherlockModule;
use chrono::prelude::*;
use fern::colors::{Color, ColoredLevelConfig};
pub use log::*;

pub enum LogLevel {
    /// corresponds to "trace!"
    Debug,
    /// corresponds to "debug!"
    Info,
    /// corresponds to "info!"
    Warning,
    /// corresponds to "warn!"
    Error,
    /// corresponds to "error!"
    Critical,
}

fn ensure_logging_dir(module: SherlockModule) -> Result<(), std::io::Error> {
    std::fs::create_dir_all("/var/log/sherlock/")?;
    std::fs::create_dir_all(format!("/var/log/sherlock/{}/", module))?;
    std::fs::create_dir_all("/var/log/sherlock/by-date/")?;

    Ok(())
}

/// initializes the logger. should be called EVERY TIME a module starts, or is interacted with in
/// anyway. it is safe to call it multiple times.
pub fn logger_init(module: SherlockModule) {
    if let Err(reason) = logger_init_helper(module) {
        if !reason
            .to_string()
            .ends_with("the logging system was already initialized")
        {
            use simple_logger::SimpleLogger;

            let _ = SimpleLogger::new().init();
            error!("failed to initiate logger properly because: {reason}");
        }
    } else {
        log::debug!("logger initiated");
    }
}

fn logger_init_helper(module: SherlockModule) -> anyhow::Result<bool> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Blue)
        .info(Color::Green)
        .warn(Color::Magenta)
        .error(Color::Red);

    let dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}{asctime} | {level} | {process} | {name} |\x1B[0m  {message}",
                color_line = format!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                asctime = format!("{}", Local::now().format("%a %h %d %H:%M:%S %Y")),
                level = record.level(),
                process = std::process::id(),
                name = record.target(),
                message = message,
            ))
        })
        .filter(|metadata| {
            // Reject messages with the `Error` log level.
            !metadata.target().starts_with("actix") && !metadata.target().starts_with("mio")
        })
        .chain(std::io::stderr());

    match ensure_logging_dir(module) {
        Err(e) if e.kind() != std::io::ErrorKind::PermissionDenied => {
            dispatch
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{asctime} | {level} | {process} | {name} |  {message}",
                        asctime = format!("{}", Local::now().format("%a %h %d %H:%M:%S %Y")),
                        level = record.level(),
                        process = std::process::id(),
                        name = record.target(),
                        message = message,
                    ))
                })
                .filter(|metadata| metadata.target().starts_with("sherlock"))
                .chain(fern::DateBased::new(
                    format!("/var/log/sherlock/{}/", module),
                    "%Y-%m-%d.log",
                ))
                .chain(fern::DateBased::new(
                    "/var/log/sherlock/by-date/",
                    "%Y-%m-%d.log",
                ))
                .apply()?;
            Ok(true)
        }
        Ok(_) | Err(_) => {
            dispatch.apply()?;
            warn!("not logging to log files, this should only be done when testing. if you are running this in prod, be warned.");
            Ok(false)
        }
    }
}

/// TODO: make macros with the same name as the LogLevel enum and use that instead, that way i can
/// preserve the part of the logs that says where the logs originated.
pub fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Debug => trace!("{}", message),
        LogLevel::Info => debug!("{}", message),
        LogLevel::Warning => info!("{}", message),
        LogLevel::Error => warn!("{}", message),
        LogLevel::Critical => error!("{}", message),
    }
}
