// Copyright 2013-2014 The rust-accountkit developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::convert::From;

use url::ParseError;
use hyper::error;
use std::io;

#[derive(Debug)]
pub enum AccountKitError {
    KindError(error::Error),
    UrlError(ParseError),
    IoError(io::Error),
}


impl From<error::Error> for AccountKitError {
    fn from(err: error::Error) -> AccountKitError {
        AccountKitError::KindError(err)
    }
}

impl From<ParseError> for AccountKitError {
    fn from(err: ParseError) -> AccountKitError {
        AccountKitError::UrlError(err)
    }
}

impl From<io::Error> for AccountKitError {
    fn from(err: io::Error) -> AccountKitError {
        AccountKitError::IoError(err)
    }
}

impl fmt::Display for AccountKitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountKitError::KindError(ref err) => write!(f, "{}", err),
            AccountKitError::UrlError(ref err) => write!(f, "{}", err),
            AccountKitError::IoError(ref err) => write!(f, "{}", err),
        }
    }
}
