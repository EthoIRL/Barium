use std::error::Error;
use std::{fmt, io};
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use crate::API_VERSION;

#[derive(Debug)]
pub enum RegistrationError {
    MismatchVersion { supplied_version: i32 },
    BadResponse,
    Unknown,
}

impl Display for RegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationError::MismatchVersion { supplied_version} => {
                write!(f, "version supplied from server client does not match governor version (Supplied: {}, Current: {})", supplied_version, API_VERSION)
            },
            RegistrationError::BadResponse => {
                write!(f, "error occurred when sending registration response")
            },
            RegistrationError::Unknown => {
                write!(f, "an unknown error occurred when handling registration")
            }
        }
    }
}

impl Into<Box<dyn Error>> for RegistrationError {
    fn into(self) -> Box<dyn Error> {
        io::Error::new(ErrorKind::Other, self.to_string()).into()
    }
}