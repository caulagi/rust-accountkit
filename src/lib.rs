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

pub use error::{UtilError, AccountKitError};
pub use util::{Util, UtilResult, get_app_access_token};
pub use accountkit::{AccountKit, AccountKitResponse, AccountKitResult};

mod error;
mod util;
mod accountkit;
