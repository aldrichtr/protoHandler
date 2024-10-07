use std::process::Command;

use regex::Regex;
use simplelog::{debug, info};

use crate::config::{Config, ProtocolConfig, ShellConfig};
use crate::error::ProtoHandlerError;

/// Builds a command based on the given URI and configuration.
///
/// This function parses the URI to determine the protocol and then looks up the
/// corresponding shell and script configuration to construct a command.
///
/// # Arguments
///
/// * `uri` - A string representing the URI.
/// * `config` - A reference to the configuration object.
///
/// # Returns
///
/// * `Ok(Command)` - If the command is successfully built.
/// * `Err(ProtoHandlerError)` - If there is an error in parsing the URI or looking up the configuration.
///
/// # Errors
///
/// This function will return an error if:
/// - The URI does not contain a recognized protocol.
/// - The protocol is not configured.
/// - The shell is not configured.
///
/// # Examples
///
/// ```
/// let uri = "http://example.com".to_string();
/// let config = Config::new();
/// let command = build_command(uri, config);
/// assert!(command.is_ok());
/// ```
pub fn build_command(uri : String, config : Config) -> Result<Command, ProtoHandlerError> {
    let proto : String;
    let program : String;
    let mut commandline : Vec<String>;
    commandline = Vec::new();

    if let Some(p) = get_protocol(&uri) {
        proto = p;
        info!("Uri contains protocol {proto}");
    } else {
        return Err(ProtoHandlerError::UriParseError { uri });
    }

    if let Some(protocol_config) = lookup_protocol(&proto, &config) {
        debug!("Found configuration for protocol '{proto}'");
        let shell_name = protocol_config.shell.name;
        debug!("Shell is configured as '{shell_name}'");

        if let Some(shell_config) = lookup_shell(&shell_name, &config) {
            program = shell_config.cmd;
            commandline.extend(shell_config.args);
        } else {
            return Err(ProtoHandlerError::ShellNotConfigured { sh : shell_name });
        }

        let extra_args = protocol_config.shell.args;
        if !extra_args.is_empty() {
            commandline.extend(extra_args);
        }

        commandline.push(protocol_config.script.name);
        let script_args = protocol_config.script.args;
        if !script_args.is_empty() {
            commandline.extend(script_args);
        }
        let mut command = Command::new(program);
        let quoted_uri = format!("\"{uri}\"");
        command.args(commandline).arg(quoted_uri);
        Ok(command)
    } else {
        info!("protocol '{proto}' not configured");
        Err(ProtoHandlerError::ProtocolNotConfigured { proto : (proto) })
    }
}

/// Extracts the protocol from the given URI.
///
/// This function uses a regular expression to find the protocol at the beginning of the URI.
///
/// # Arguments
///
/// * `uri` - A reference to a string representing the URI.
///
/// # Returns
///
/// * `Some(String)` - If the protocol is found.
/// * `None` - If the protocol is not recognized.
///
/// # Examples
///
/// ```
/// let uri = "http://example.com".to_string();
/// let protocol = get_protocol(&uri);
/// assert_eq!(protocol, Some("http".to_string()));
/// ```

#[must_use] pub fn get_protocol(uri : &String) -> Option<String> {
    // attempt to find the protocol string at the begining of the uri
    let re = Regex::new(r"^(?<proto>[a-z][a-zA-Z0-9-_]+):\/\/").unwrap();
    if let Some(p) = re.captures(uri) {
        let proto = p["proto"].to_string();
        Some(proto)
    } else {
        info!("protocol not recognized {uri}");
        None
    }
}

/// Looks up the protocol configuration in the given configuration object.
///
/// This function searches for the protocol configuration by name.
///
/// # Arguments
///
/// * `proto` - A reference to a string representing the protocol name.
/// * `config` - A reference to the configuration object.
///
/// # Returns
///
/// * `Some(ProtocolConfig)` - If the protocol configuration is found.
/// * `None` - If the protocol configuration is not found.
///
/// # Examples
///
/// ```
/// let proto = "http".to_string();
/// let config = Config::new();
/// let protocol_config = lookup_protocol(&proto, &config);
/// assert!(protocol_config.is_some());
/// ```

#[must_use] pub fn lookup_protocol(proto : &String, config : &Config) -> Option<ProtocolConfig> {
    config
        .protocols
        .clone()
        .into_iter()
        .find(|p| p.name == *proto)
}

/// Looks up the shell configuration in the given configuration object.
///
/// This function searches for the shell configuration by name.
///
/// # Arguments
///
/// * `name` - A reference to a string representing the shell name.
/// * `config` - A reference to the configuration object.
///
/// # Returns
///
/// * `Some(ShellConfig)` - If the shell configuration is found.
/// * `None` - If the shell configuration is not found.
///
/// # Examples
///
/// ```
/// let name = "bash".to_string();
/// let config = Config::new();
/// let shell_config = lookup_shell(&name, &config);
/// assert!(shell_config.is_some());
/// ```

#[must_use] pub fn lookup_shell(name : &String, config : &Config) -> Option<ShellConfig> {
    debug!("Looking up configuration for shell '{name}'");
    config.shells.clone().into_iter().find(|s| s.name == *name)
}
