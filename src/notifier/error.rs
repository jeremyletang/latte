// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::fmt;

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum Error {
    HourOutOfRange,
    MinuteOutOfRange,
    UnknownMessageId,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HourOutOfRange => "Hour out of range (must be between 0-23 inclusive).",
            Error::MinuteOutOfRange => "Minute out of range (must be between 0-59 inclusive).",
            Error::UnknownMessageId => "Unknown message id.",
        }
    }

    fn cause(&self) -> Option<&StdError> { None }
}
