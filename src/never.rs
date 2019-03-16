use std::{error::Error, fmt};

/// A type that can't be constructed.
#[derive(Clone, Copy)]
pub enum Never {}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl fmt::Debug for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl Error for Never {
    fn description(&self) -> &str {
        match *self {}
    }
}
