// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::convert::From;
use std::error;
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidChannel,
    InvalidParameter,
    NotImplemented,
    WatchdogTimeout,
    AccessFailed(std::io::Error),
    ParseIntError,
    GenericError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidChannel => write!(f, "Invalid channel specified"),
            Error::InvalidParameter => write!(f, "Invalid parameter specified"),
            Error::NotImplemented => write!(f, "Functionality is not implemented"),
            Error::WatchdogTimeout => write!(f, "Watchdog timed out"),
            Error::AccessFailed(_) => write!(f, "Failed to access device"),
            Error::ParseIntError => write!(f, "Failed to convert number"),
            Error::GenericError => write!(f, "Generic internal error"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::InvalidChannel => None,
            Error::InvalidParameter => None,
            Error::NotImplemented => None,
            Error::WatchdogTimeout => None,
            Error::AccessFailed(ref err) => Some(err),
            Error::ParseIntError => None,
            Error::GenericError => None,
        }
    }
}

impl Error {
    pub fn generic_access_error() -> Error {
        Error::AccessFailed(io::Error::from(io::ErrorKind::Other))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::AccessFailed(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Error {
        Error::ParseIntError
    }
}
