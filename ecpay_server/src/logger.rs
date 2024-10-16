use chrono::Local;
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

pub fn init() {
    // 獲取當前日期
    let date = Local::now().format("%Y-%m-%d").to_string();

    // Console Appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} - {l} - {M}::{f}:{L} - {m}{n}",
        )))
        .build();

    // 滾動策略：當檔案達到 1MB 時滾動，最多保留 10 個檔案 (Info 級別)
    let info_roller = FixedWindowRoller::builder()
        .base(1)
        .build(&format!("logs/info/info-{}-{{}}.log", date), 10)
        .unwrap();
    let info_trigger = SizeTrigger::new(1 * 1024 * 1024); // 1MB 檔案大小限制
    let info_policy = CompoundPolicy::new(Box::new(info_trigger), Box::new(info_roller));
    let info_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} - {l} - {M}::{f}:{L} - {m}{n}",
        )))
        .build("logs/info/info.log", Box::new(info_policy))
        .unwrap();

    // 滾動策略：當檔案達到 2MB 時滾動，最多保留 5 個檔案 (Warn 級別)
    let warn_roller = FixedWindowRoller::builder()
        .base(1)
        .build(&format!("logs/warn/warn-{}-{{}}.log", date), 5)
        .unwrap();
    let warn_trigger = SizeTrigger::new(2 * 1024 * 1024); // 2MB 檔案大小限制
    let warn_policy = CompoundPolicy::new(Box::new(warn_trigger), Box::new(warn_roller));
    let warn_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} - {l} - {M}::{f}:{L} - {m}{n}",
        )))
        .build("logs/warn/warn.log", Box::new(warn_policy))
        .unwrap();

    // 滾動策略：當檔案達到 2MB 時滾動，最多保留 1 個檔案 (Error 級別)
    let error_roller = FixedWindowRoller::builder()
        .base(1)
        .build(&format!("logs/error/error-{}-{{}}.log", date), 1)
        .unwrap();
    let error_trigger = SizeTrigger::new(2 * 1024 * 1024); // 2MB 檔案大小限制
    let error_policy = CompoundPolicy::new(Box::new(error_trigger), Box::new(error_roller));
    let error_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} - {l} - {M}::{f}:{L} - {m}{n}",
        )))
        .build("logs/error/error.log", Box::new(error_policy))
        .unwrap();

    // 配置根日誌，只寫入 INFO 級別
    let root = Root::builder()
        .appender("stdout")
        .appender("info_file")
        .build(LevelFilter::Info);

    // `warn` logger，將 `warn` 級別的訊息寫入 `warn_file`
    let warn_logger = Logger::builder()
        .appender("warn_file")
        .additive(false)
        .build("warn_logger", LevelFilter::Warn);

    // `error` logger，將 `error` 級別的訊息寫入 `error_file`
    let error_logger = Logger::builder()
        .appender("error_file")
        .additive(false)
        .build("error_logger", LevelFilter::Error);

    // 建立 config，並將不同的日誌級別 logger 連接到正確的檔案
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("info_file", Box::new(info_appender)))
        .appender(Appender::builder().build("warn_file", Box::new(warn_appender)))
        .appender(Appender::builder().build("error_file", Box::new(error_appender)))
        .logger(warn_logger)
        .logger(error_logger)
        .build(root)
        .unwrap();

    // 初始化 log4rs
    log4rs::init_config(config).unwrap();
}
