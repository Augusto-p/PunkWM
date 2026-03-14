use std::fmt;

#[derive(Debug)]
pub enum NmcliError {
    NmcliNotInstalled,
    CommandFailed(String),
}
// ParseError(String),

impl fmt::Display for NmcliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NmcliNotInstalled => write!(f, "nmcli no está instalado"),
            Self::CommandFailed(e) => write!(f, "nmcli falló: {e}"),
        }
    }
}
// Self::ParseError(e) => write!(f, "error parseando salida: {e}"),

impl std::error::Error for NmcliError {}
