//! This module provides configuration management for the `protohandler` application.
//!
//! It includes structures and functions for handling logging, shell configurations,
//! and protocol configurations. The configuration is serialized and deserialized
//! using YAML format.

use std::{path::PathBuf, str::FromStr};

use etcetera::BaseStrategy;
use log::error;
use serde::{Deserialize, Serialize};
use serde_yml;
use simplelog::info;
use simplelog::LevelFilter;

use crate::error::ProtoHandlerError;

// APP_NAME and CONFIG_EXT are used to determine the directory and file name of
// the serialized Config
const APP_NAME: &str = "protohandler";
const CONFIG_EXT: &str = ".yml";

// --------------------------------------------------------------------------------
// region: Config
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
/// Represents the configuration for the `protohandler` application.
pub struct Config {
    /// Logging configuration.
    pub logging: LoggingConfig,
    /// List of shell configurations.
    pub shells: Vec<ShellConfig>,
    /// List of protocol configurations.
    pub protocols: Vec<ProtocolConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let pwsh = ShellConfig {
            name: String::from("pwsh"),
            cmd: String::from("pwsh"),
            args: vec![
                String::from("-noProfile"),
                String::from("-noLogo"),
                String::from("-File"),
            ],
        };
        let python = ShellConfig {
            name: String::from("python"),
            cmd: String::from("python"),
            args: Vec::new(),
        };

        Self {
            logging: LoggingConfig::default(),
            shells: vec![pwsh, python],
            protocols: Vec::new(),
        }
    }
}

impl Config {
    #[must_use] pub fn new() -> Self {
        Config::default()
    }

    /// Returns the directory where the `protohandler` configuration is stored.
    #[must_use] pub fn get_directory(&self) -> PathBuf {
        let strategy = etcetera::choose_base_strategy().expect("Unable to find config directory");

        strategy.config_dir().join(APP_NAME)
    }

    /// Returns the file path of the `protohandler` configuration file.
    #[must_use] pub fn get_file(&self) -> PathBuf {
        let dir = self.get_directory();
        let file = APP_NAME.to_string() + CONFIG_EXT;

        dir.join(file)
    }

    /// Loads the configuration from the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the path to the configuration file.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the configuration is successfully loaded.
    /// * `Err(ProtoHandlerError)` - If there is an error in loading the configuration.
    pub fn load(&mut self, path: String) -> Result<(), ProtoHandlerError> {
        match PathBuf::from_str(&path) {
            Ok(file) => {
                if let Ok(content) = std::fs::read_to_string(file) {
                    info!("Loading configuration from {path}");
                    let config: Config = serde_yml::from_str(&content).unwrap();
                    self.logging = config.logging;
                    self.shells = config.shells;
                    self.protocols = config.protocols;
                    Ok(())
                } else {
                    error!("Could not load config file {path}");
                    Err(ProtoHandlerError::ConfigParseError { path })
                }
            }
            Err(_e) => Err(ProtoHandlerError::PathError { path }),
        }
    }
}

// endregion Config
// --------------------------------------------------------------------------------

// --------------------------------------------------------------------------------
// region: Logging config

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Represents the logging levels for the `protohandler` application.
pub enum LoggingLevel {
    Debug,
    Info,
    Warn,
    Error,
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
/// Represents the logging configuration for the `protohandler` application.
pub struct LoggingConfig {
    /// Path to the log file.
    pub path: String,
    /// Logging level.
    pub level: LoggingLevel,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            path: String::from("protocolhandler.log"),
            level: LoggingLevel::Info,
        }
    }
}

impl LoggingConfig {
    /// Converts a `protohandler` logging level to a `simplelog` logging level.
    ///
    /// # Returns
    ///
    /// * `LevelFilter` - The corresponding `simplelog` logging level.
    #[must_use] pub fn convert_level(&self) -> LevelFilter {
        match self.level {
            LoggingLevel::Debug => LevelFilter::Debug,
            LoggingLevel::Info => LevelFilter::Info,
            LoggingLevel::Warn => LevelFilter::Warn,
            LoggingLevel::Error => LevelFilter::Error,
            LoggingLevel::None => LevelFilter::Off,
        }
    }
}
// endregion Logging config
// --------------------------------------------------------------------------------

// --------------------------------------------------------------------------------
// region: Shells config

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[derive(Default)]
/// Represents the configuration for a shell in the `protohandler` application.
pub struct ShellConfig {
    /// Name of the shell.
    pub name: String,
    /// Command to execute the shell.
    pub cmd: String,
    /// Arguments for the shell command.
    pub args: Vec<String>,
}

// endregion Shells config
// --------------------------------------------------------------------------------

// --------------------------------------------------------------------------------
// region: Protocols config

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[derive(Default)]
/// Represents the configuration for a protocol in the `protohandler` application
///
/// Example
pub struct ProtocolConfig {
    /// Name of the protocol
    pub name : String,
    /// A short description of the protocol
    pub desc : String,
    /// The script to call
    pub script : ProtocolScriptConfig,
    /// The shell to use when calling the script
    pub shell : ProtocolShellConfig,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[derive(Default)]
pub struct ProtocolScriptConfig {
    pub name : String,
    pub args : Vec<String>,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[derive(Default)]
pub struct ProtocolShellConfig {
    pub name : String,
    pub args : Vec<String>
}


// endregion Protocols config
// --------------------------------------------------------------------------------
