use clap::Parser;

// URI = scheme ":" ["//" authority] path ["?" query] ["#" fragment]
// URI = proto :// subcommand ? payload
// payload = name=val&name=val

// URI components
// snip-proto://capture?template=c&url=https://docs.github.com/en/get-started&title=Get started&body=get started
//  - proto : snip-proto
//  - subcommand : capture
//  - payload :
//    - template = c
//    - url = https://docs.github.com/en/get-started
//    - title = 'Get started'
//    - body = 'get started'

#[derive(Debug, Parser)]
#[command(version)]
pub(crate) struct Cli {
    /// Register a new protocol
    ///
    /// Add a protocol that will be handled by protoHandle.rs
    #[arg(
        group = "registration",
        requires = "script_path",
        short = 'r',
        long = "register-protocol"
    )]
    pub new_proto : Option<String>,

    /// The path to the script to run
    ///
    /// The script to associate with the given protocol.  Path can be given as
    /// relative to 'script-directory' or as a fully-qualified path
    #[arg(group = "registration", short = 's', long = "script-path")]
    pub script_path : Option<String>,

    /// The URI to be processed.
    ///
    /// The URI to process by protoHandle.rs.  Which script or process is
    /// determined by the URI protocol-scheme
    #[arg(group = "input", short = 'u', long = "uri")]
    pub uri : Option<String>,

    /// Alternate configuration file to use
    ///
    /// Use an alternate configuration file instead of the default
    #[arg(short = 'c', long = "config-file")]
    pub config_file : Option<String>,

    /// Alternate log file to write to
    ///
    /// Depending on the verbosity settings, send log messages to an alternate
    /// file, instead of the default.
    #[arg(short = 'l', long = "log-file")]
    pub log_file : Option<String>,
}
