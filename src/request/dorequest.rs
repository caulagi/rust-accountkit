// Copyright 2013-2014 The giangnh developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::result::Result;
use std::io::Read;
use std::fmt;
use std::time::Duration;

use error::AccountKitError;

use hyper::method::Method;
use hyper::client::request;
use hyper::Url;

pub trait BaseRequest {
    fn url(&self) -> &str;
    fn method(&self) -> Method;
}

#[derive(Debug)]
pub struct AccountKitResponse {
    pub body: String,
    pub status_code: u16,
    pub canonical_reason: Option<&'static str>,
}

impl fmt::Display for AccountKitResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "body: {}\nstatus_code: {} \ncanonical_reason: {} \n
               ",
               self.body,
               self.status_code,
               if self.canonical_reason.is_some() {
                   self.canonical_reason.unwrap()
               } else {
                   "None"
               })
    }
}

pub trait DoRequest: BaseRequest {
    fn retriev_json(&self) -> Result<AccountKitResponse, AccountKitError> {
        let duration = Duration::from_secs(10);
        let url = try!(Url::parse(self.url()));
        let fresh_net = try!(request::Request::new(self.method(), url));
        try!(fresh_net.set_read_timeout(Some(duration)));
        try!(fresh_net.set_write_timeout(Some(duration)));
        let streaming_req = try!(fresh_net.start());
        let mut response = try!(streaming_req.send());
        let mut s = String::new();
        let status = response.status;
        let status_code = status.to_u16();
        let reason = status.canonical_reason();
        try!(response.read_to_string(&mut s));
        Ok(AccountKitResponse {
            body: s,
            status_code: status_code,
            canonical_reason: reason,
        })
    }
    fn retriev(&self) -> Result<AccountKitResponse, AccountKitError> {
        let result = try!(self.retriev_json());
        Ok(result)
    }
}
