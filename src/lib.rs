// Copyright 2013-2015 The rust-accountkit developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! rust-accountkit is an implementation of the [Facebook Accountkit](https://developers.facebook.com/docs/accountkit).
//!
//! It builds with [Cargo](http://crates.io/).
//!
//!

extern crate hyper;
extern crate url;

pub use accountkit::AccountKit;
pub use request::RequestBuilder;
pub use error::AccountKitError;

mod accountkit;
mod request;
mod error;
