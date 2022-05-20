use chrono::prelude::*;

enum Level {
    Trace,
    Info,
    Warn,
    Error,
    Fatal,
}

enum Source {
    Core,
    App,
}

// Client

pub fn trace(msg: &str) {
    log_app(Level::Trace, msg);
}

pub fn info(msg: &str) {
    log_app(Level::Info, msg);
}

pub fn warn(msg: &str) {
    log_app(Level::Warn, msg);
}

pub fn error(msg: &str) {
    log_app(Level::Error, msg);
}

pub fn fatal(msg: &str) {
    log_app(Level::Fatal, msg);
}

// Core

pub(crate) fn core_trace(msg: &str) {
    log_core(Level::Trace, msg);
}

pub(crate) fn core_info(msg: &str) {
    log_core(Level::Info, msg);
}

pub(crate) fn core_warn(msg: &str) {
    log_core(Level::Warn, msg);
}

pub(crate) fn core_error(msg: &str) {
    log_core(Level::Error, msg);
}

pub(crate) fn core_fatal(msg: &str) {
    log_core(Level::Fatal, msg);
}

fn log_app(level: Level, msg: &str) {
    match level {
        Level::Error => eprintln!("{} {}", timestamp_info(level, Source::App), msg),
        _ => println!("{} {}", timestamp_info(level, Source::App), msg),
    }
}

fn log_core(level: Level, msg: &str) {
    match level {
        Level::Error => eprintln!("{} {}", timestamp_info(level, Source::Core), msg),
        _ => println!("{} {}", timestamp_info(level, Source::Core), msg),
    }
}

fn timestamp_info(level: Level, source: Source) -> String {
    // let time = Local::now().format("%H:%M:%S%.3f").to_string();
    let time = Local::now().format("%H:%M:%S").to_string();

    let source_tag = match source {
        Source::App => "APP".to_string(),
        Source::Core => "CORE".to_string(),
    };

    let level_tag = match level {
        Level::Trace => "trace".to_string(),
        Level::Info => "info".to_string(),
        Level::Warn => "warn".to_string(),
        Level::Error => "error".to_string(),
        Level::Fatal => "fatal".to_string(),
    };

    format!(
        "[{}] [{}] [{}]",
        time,
        source_tag,
        level_tag
    )
}