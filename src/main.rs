pub mod cli;
pub mod command;
pub mod config;
pub mod error;

use std::borrow::Cow;
use std::path::Path;

extern crate log;
extern crate simplelog;

use std::ffi::OsStr;
use std::fs::File;
use std::process::Stdio;
use std::vec;

use clap::Parser;
use command::build_command;
use resolve_path::PathResolveExt;
use simplelog::CombinedLogger;
use simplelog::*;

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
        match Path::new(&config_file).resolve().exists() {
            true => {
                println!("Using {:?} as config file", config_file);
                match config.load(config_file) {
                    Ok(_) => {
                        println!("Successfully loaded config");
                    },
                    Err(e) => {
                        panic!("Error loading config {}", e);
                    },
                }
            },
            false => {
                println!("{:?} given as config file but does not exist", config_file);
            },
        }
    };

    if let Some(log_file) = args.log_file {
        match Path::new(&log_file).exists() {
            true => {
                println!("Using {:?} as log file", log_file);
                config.logging.path = log_file;
            },
            false => {
                println!("{:?} given as log file but does not exist yet", log_file);
            },
        }
    }
    init_log(&config);
    match args.uri {
        None => (),
        Some(uri) => {
            info!("Got uri, building command");

            let child = build_command(uri, config);
            match child {
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
                    panic!("Error spawning process {}", e)
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
