// Copyright 2013-2015 The rust-accountkit developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! rust-accountkit is an implementation of the [Facebook Accountkit](https://developers.facebook.com/docs/accountkit).
//!
//! # Example
//! ```rust,ignore
//! let app_token = format!("AA|{}|{}", "facebook_app_id", "account_kit_client_secret");
//!
//! let account_kit = AccountKit::with_token(app_token.as_str(), None);
//! // If you have enabled the Require App Secret setting in your app's dashboards, most calls that accept an account access token as a parameter will now require an additional appsecret_proof parameter to verify that these calls are coming from your own servers.
//! // app secret proof is a sha256 hash of your access token, using the app secret as the key
//! let account_kit = AccountKit::with_token(app_token.as_str(), "appsecret_proof_key");
//! let response = account_kit.authorization_code("authorization_code").retriev();
//! match response {
//!  Ok(user_token) => println!("{:?}", user_token),
//!  Err(err) => println!("{}", err.to_string())
//! }
//! ```

extern crate hyper;
extern crate url;

pub use accountkit::AccountKit;
pub use request::RequestBuilder;
pub use request::dorequest::AccountKitResponse;
pub use error::AccountKitError;

mod accountkit;
mod request;
mod error;
