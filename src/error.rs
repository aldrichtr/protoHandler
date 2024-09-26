use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtoHandlerError {
    #[error("Path '{path}' was not found")]
    PathError { path : String },

    #[error("Could not determine protocol in uri: '{uri}'")]
    UriParseError { uri : String },

    #[error("{proto} Protocol not configured")]
    ProtocolNotConfigured { proto : String },

    #[error("Shell {sh} not configured")]
    ShellNotConfigured { sh : String },

    #[error("Could not parse config file '{path}'")]
    ConfigParseError { path : String }
}
