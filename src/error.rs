// Copyright 2013-2014 The rust-accountkit developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate url;
extern crate hyper;

use std::fmt;
use std::convert::From;

use self::url::ParseError;
use self::hyper::error;

#[derive(Debug)]
pub enum UtilError {
    UrlError(ParseError),
    TupleError(()),
}

impl From<ParseError> for UtilError {
    fn from(err: ParseError) -> UtilError {
        UtilError::UrlError(err)
    }
}

impl From<()> for UtilError {
    fn from(err: ()) -> UtilError {
        UtilError::TupleError(err)
    }
}

impl fmt::Display for UtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UtilError::UrlError(ref err) => write!(f, "{}", err),
            UtilError::TupleError(()) => write!(f, "{}", "URL is cannot-be-a-base"),
        }
    }
}

#[derive(Debug)]
pub enum AccountKitError {
    KindError(error::Error),
}


impl From<error::Error> for AccountKitError {
    fn from(err: error::Error) -> AccountKitError {
        AccountKitError::KindError(err)
    }
}

impl fmt::Display for AccountKitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountKitError::KindError(ref err) => write!(f, "{}", err),
        }
    }
}
