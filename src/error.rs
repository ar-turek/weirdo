use postgres::error::Error as PostgresError;
use std::error;
use std::fmt;
use std::io::Error as IoError;
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Config(TomlError),
    Connection(PostgresError),
    Db(PostgresError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
            Error::Config(ref err) => write!(f, "Config parse error: {}", err),
            Error::Connection(ref err) => write!(f, "Connection error: {}", err),
            Error::Db(ref err) => write!(f, "Database error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Config(ref err) => Some(err),
            Error::Connection(ref err) => Some(err),
            Error::Db(ref err) => Some(err),
        }
    }
}

impl std::convert::From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl std::convert::From<TomlError> for Error {
    fn from(err: TomlError) -> Error {
        Error::Config(err)
    }
}

impl std::convert::From<PostgresError> for Error {
    fn from(err: PostgresError) -> Error {
        if let Some(code) = err.code() {
            match code.code() {
                // Postgres connection errors SQLSTATEs
                "08000" | "08003" | "08006" | "08001" | "08004" | "08007" | "08P01" => {
                    Error::Connection(err)
                }
                _ => Error::Db(err),
            }
        } else {
            Error::Connection(err)
        }
    }
}
