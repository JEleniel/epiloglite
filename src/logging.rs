use std::{fs::create_dir_all, path::PathBuf};

use chrono::SecondsFormat;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;

pub fn init_logging() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                .trace(Color::BrightBlack)
                .debug(Color::White)
                .info(Color::BrightWhite)
                .warn(Color::BrightYellow)
                .error(Color::Red);
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now()
                    .to_rfc3339_opts(SecondsFormat::Millis, true)
                    .as_str(),
                colors.color(record.level()),
                record.target(),
                message
            ));
        })
        .chain(std::io::stdout())
        .level(log::LevelFilter::Info)
        .apply()
        .unwrap();

    info!(target: module_path!(), "----- Logging started -----");
}
