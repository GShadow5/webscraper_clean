use crate::CONFIG;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
//use crate::CONFIG;

pub fn initialize_logging() -> log4rs::Handle {
    // Initialize logging

    let level: LevelFilter = match &CONFIG.verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({l})}: {m}{n}")))
        .build(); // This appender is filtered, but only later

    let roller = FixedWindowRoller::builder()
        .build("log/my{}.log", 50)
        .unwrap();
    let policy: CompoundPolicy =
        CompoundPolicy::new(Box::new(SizeTrigger::new(50 * 1024)), Box::new(roller));
    let file_logger = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}",
        )))
        .build("log/my.log", Box::new(policy))
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level))) // This is the filter
                .build("stdout", Box::new(stdout)),
        )
        .appender(Appender::builder().build("file_logger", Box::new(file_logger)))
        .logger(
            Logger::builder()
                .additive(false) // If additive is true, you get double output from the stdout appender
                .appender("stdout")
                .appender("file_logger")
                .build("w10s_webscraper", LevelFilter::Trace),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();
    let handle = log4rs::init_config(config).unwrap();
    handle
}
