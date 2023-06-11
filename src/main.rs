pub mod motd_parser;
pub mod constants;
pub mod container;
pub mod settings;
pub mod bedrock;
pub mod discord;
pub mod helper;
pub mod logger;
pub mod java;
pub mod web;

use signal_hook::{ iterator::Signals, consts::{ SIGINT, SIGTERM } };
use simplelog::{ error, info, debug, warn, trace };
use std::{ thread, process::exit };
use logger::init_logger;
use clap::{ Arg, Command, ArgAction, value_parser };

use crate::logger::LoggerSettings;

fn main() {
  let mut logger_settings = LoggerSettings {
    level: simplelog::LevelFilter::Info,
    file_level: simplelog::LevelFilter::Debug,
  };

  let level: simplelog::LevelFilter;
  let file_level: simplelog::LevelFilter;

  let before_help = format!(
    "{} {}\nLicense: {}\nSource: {}\nAuthors: {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_LICENSE"),
    env!("CARGO_PKG_HOMEPAGE"),
    env!("CARGO_PKG_AUTHORS").split(':').collect::<Vec<&str>>().join(", ")
  );

  let matches = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .before_help(before_help)
    .name(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(
      Arg::new("trace")
        .long("trace")
        .num_args(0)
        .default_value("false")
        .value_parser(value_parser!(bool))
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["debug", "error", "warn", "info", "off"])
        .help("Sets the logging level to trace")
    )
    .arg(
      Arg::new("debug")
        .long("debug")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "error", "warn", "info", "off"])
        .help("Sets the logging level to debug")
    )
    .arg(
      Arg::new("error")
        .long("error")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "warn", "info", "off"])
        .help("Sets the logging level to error")
    )
    .arg(
      Arg::new("warn")
        .long("warn")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "error", "info", "off"])
        .help("Sets the logging level to warn")
    )
    .arg(
      Arg::new("info")
        .long("info")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "error", "warn", "off"])
        .help("Sets the logging level to info (default)")
    )
    .arg(
      Arg::new("off")
        .long("off")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "error", "warn", "info"])
        .help("Sets the logging level to off")
    )
    .arg(
      Arg::new("trace-file")
        .long("trace-file")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with("disable-log")
        .help("Sets the logging level to trace for the log file")
    )
    .arg(
      Arg::new("disable-log")
        .long("disable-log")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with("trace-file")
        .help("Disables the log file")
    )
    .get_matches();

  if matches.get_flag("trace") {
    logger_settings.level = simplelog::LevelFilter::Trace;
    level = simplelog::LevelFilter::Trace;
  } else if matches.get_flag("debug") {
    logger_settings.level = simplelog::LevelFilter::Debug;
    level = simplelog::LevelFilter::Debug;
  } else if matches.get_flag("warn") {
    logger_settings.level = simplelog::LevelFilter::Warn;
    level = simplelog::LevelFilter::Warn;
  } else if matches.get_flag("error") {
    logger_settings.level = simplelog::LevelFilter::Error;
    level = simplelog::LevelFilter::Error;
  } else if matches.get_flag("off") {
    logger_settings.level = simplelog::LevelFilter::Off;
    level = simplelog::LevelFilter::Off;
  } else {
    level = simplelog::LevelFilter::Info;
  }

  if matches.get_flag("trace-file") {
    logger_settings.file_level = simplelog::LevelFilter::Trace;
    file_level = simplelog::LevelFilter::Trace;
  } else if matches.get_flag("disable-log") {
    logger_settings.file_level = simplelog::LevelFilter::Off;
    file_level = simplelog::LevelFilter::Off;
  } else {
    file_level = simplelog::LevelFilter::Debug;
  }

  init_logger(logger_settings);

  match level {
    simplelog::LevelFilter::Trace => info!("TRACE calls logging to terminal"),
    simplelog::LevelFilter::Debug => info!("DEBUG calls logging to terminal"),
    simplelog::LevelFilter::Info => info!("INFO calls logging to terminal"),
    simplelog::LevelFilter::Warn => info!("WARN calls logging to terminal"),
    simplelog::LevelFilter::Error => info!("ERROR calls logging to terminal"),
    simplelog::LevelFilter::Off => info!("Disabled logging to terminal"),
  }

  match file_level {
    simplelog::LevelFilter::Trace => info!("TRACE calls logging to file"),
    simplelog::LevelFilter::Debug => info!("DEBUG calls logging to file"),
    simplelog::LevelFilter::Off => info!("Disabled logging to file"),
    _ => (),
  }

  trace!("Hello, world!");
  debug!("Hello, world!");
  info!("Hello, world!");
  warn!("Hello, world!");
  error!("Hello, world!");

  let mut signals: signal_hook::iterator::SignalsInfo = Signals::new(&[SIGINT, SIGTERM]).unwrap();

  thread::spawn(move || {
    for sig in signals.forever() {
      match sig {
        SIGINT => info!("Received SIGINT"),
        SIGTERM => info!("Received SIGTERM"),
        _ => unreachable!(),
      }
      exit(0);
    }
  });
}
