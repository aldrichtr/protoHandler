//! A tool to trigger a script based on the protocol and URI sent to the program
//!
//! Protocol handlers are a function found in linux, `MacOS` and Windows that
//! allows the OS to associate a URI with a given program.  protoHandler.rs can
//! be configured to receive the URI and pass them on to other programs or
//! scripts based on the protocol.

#![warn(clippy::pedantic)]

pub mod cli;
pub mod runner;
pub mod config;
pub mod error;

#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::path::Path;

extern crate log;
extern crate simplelog;

use std::ffi::OsStr;
use std::fs::File;
use std::process::Stdio;
use std::vec;

use clap::Parser;
use runner::build_command;
use resolve_path::PathResolveExt;
use simplelog::CombinedLogger;
use simplelog::{ColorChoice, TermLogger, TerminalMode, WriteLogger, error, info, warn};

use crate::cli::Cli;
use crate::config::Config;

fn main() {
    let mut config = Config::new();

    let args = match Cli::try_parse() {
        Ok(args) => {
            println!("Parsed commandline arguments");
            args
        },
        Err(e) => {
            let _ = e.print();
            std::process::exit(1);
        },
    };

    if let Some(config_file) = args.config_file {
        if Path::new(&config_file).resolve().exists() {
                println!("Using {config_file:?} as config file");
                match config.load(config_file) {
                    Ok(()) => {
                        println!("Successfully loaded config");
                    },
                    Err(e) => {
                        panic!("Error loading config {e}");
                    },
                }
        } else {
            println!("{config_file:?} given as config file but does not exist");
        }
    } else {
        let default_config = config.get_file();
        if default_config.exists() {
            match config.load(default_config.into_os_string().into_string().unwrap()) {
                Ok(()) => {},
                Err(e) => {
                    panic!("Error loading config {e}");
                }
            }
        }
    }

    if let Some(log_file) = args.log_file {
        if Path::new(&log_file).exists() {
                println!("Using {log_file:?} as log file");
                config.logging.path = log_file;
            } else {
                println!("{log_file:?} given as log file but does not exist yet");
            }
    }

    init_log(&config);

    match args.uri {
        None => (),
        Some(uri) => {
            info!("Got uri, building command");

            match build_command(uri, config) {
                Ok(mut child) => {
                    let a : Vec<&OsStr> = child.get_args().collect();
                    let a = a
                        .into_iter()
                        .map(|o| o.to_string_lossy())
                        .collect::<Vec<Cow<str>>>()
                        .join(" ");
                    info!("Arguments are {a}");
                    child.stdout(Stdio::inherit());

                    // Run the external cmd
                    let result = child.spawn();

                    match result {
                        Ok(mut r) => {
                            if let Some(stdout) = r.stdout.take() {
                                info!("command output {:#?}", stdout);
                            }
                            match r.try_wait() {
                                Ok(Some(status)) => info!("exited with: {status}"),
                                Ok(None) => {
                                    info!("status not ready yet, let's really wait");
                                    let r = r.wait();
                                    info!("result: {r:?}");
                                }
                                Err(e) => info!("error attempting to wait: {e}"),
                            }
                        },
                        Err(_e) => {
                            error!("Failed to run child process");
                        }
                    }
                },
                Err(e) => {
                    panic!("Error spawning process {e}")
                },
            }
        },
    }
}

fn init_log(config : &Config) {
    // TODO: Add the fields in the simplelog Config to our config and provide an
    // 'into()'
    let log_config = simplelog::Config::default();
    let level = config.logging.convert_level();
    let log_path = config.logging.path.resolve();
    CombinedLogger::init(vec![
        TermLogger::new(
            level,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(level, log_config.clone(), File::create(log_path).unwrap()),
    ])
    .unwrap();
}
