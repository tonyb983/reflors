// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    fmt::{Error as FmtError, Result as FmtResult, Write as FmtWrite},
    io::{Error as IoError, Result as IoResult, Write as IoWrite},
    str::Utf8Error,
    string::FromUtf8Error,
};

/// Error type used by [`Writer`].
#[derive(Debug)]
pub enum Error {
    /// An error resulting from `std::fmt` functions.
    Format(FmtError),
    /// An error resulting from `std::io` functions.
    Io(IoError),
    /// An error resulting from a failed conversion from `[u8]` to `str`.
    Utf8(Utf8Error),
    /// Any other error that is not specified, along with a message explaining what occurred.
    Other(String),
}

impl From<Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Utf8(err.utf8_error())
    }
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Self {
        Error::Format(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<Error> for IoError {
    fn from(err: Error) -> Self {
        use std::io::ErrorKind;
        match err {
            Error::Format(err) => IoError::new(ErrorKind::Other, err),
            Error::Io(err) => err,
            Error::Utf8(err) => IoError::new(ErrorKind::Other, err),
            Error::Other(err) => IoError::new(ErrorKind::Other, err),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Other(err)
    }
}

impl From<&String> for Error {
    fn from(err: &String) -> Self {
        Error::Other(err.clone())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Other(err.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        match self {
            Error::Format(err) => err.fmt(f),
            Error::Io(err) => err.fmt(f),
            Error::Utf8(err) => err.fmt(f),
            Error::Other(err) => write!(f, "an unknown error has occurred: {}", err),
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::Format(err) => Error::Format(*err),
            Error::Io(err) => Error::Io(IoError::new(err.kind(), err.to_string())),
            Error::Utf8(err) => Error::Utf8(*err),
            Error::Other(err) => Error::Other(err.clone()),
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::Format(err), Error::Format(err2)) => err == err2,
            (Error::Io(err), Error::Io(err2)) => {
                err.kind() == err2.kind() && err.to_string() == err2.to_string()
            }
            (Error::Utf8(err), Error::Utf8(err2)) => err == err2,
            (Error::Other(err), Error::Other(err2)) => err == err2,
            _ => false,
        }
    }
}

impl Eq for Error {}

impl std::error::Error for Error {}

/// Result type used by [`Writer`].
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;
