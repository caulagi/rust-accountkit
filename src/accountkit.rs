// Copyright 2013-2014 The rust-accountkit developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


extern crate hyper;

use self::hyper::client::Client;
use std::result::Result;
use error::AccountKitError;
use std::io::Read;

pub type AccountKitResult = Result<AccountKitResponse, AccountKitError>;

#[derive(Debug)]
pub struct AccountKitResponse {
    body: String,
    status_code: u16,
    canonical_reason: Option<&'static str>,
}

#[derive(Debug)]
pub struct AccountKit {
    client: Client,
}

impl AccountKit {
    pub fn new() -> AccountKit {
        let client = Client::new();
        AccountKit { client: client }
    }

    fn send_request<'a>(&self, url: &'a str) -> AccountKitResult {
        let mut response = try!(self.client.get(url).send());
        let status = response.status;
        let status_code = status.to_u16();
        let reason = status.canonical_reason();
        let mut body = String::new();
        response.read_to_string(&mut body);
        Ok(AccountKitResponse {
            body: body,
            status_code: status_code,
            canonical_reason: reason,
        })
    }

    pub fn user_authorization<'a>(&self, url: &'a str) -> AccountKitResult {
        let response = try!(self.send_request(url));
        Ok(response)
    }
    pub fn validate_access_token<'a>(&self, url: &'a str) -> AccountKitResult {
        let response = try!(self.send_request(url));
        Ok(response)
    }

    pub fn remove_account<'a>(&self, url: &'a str) -> AccountKitResult {
        let mut response = try!(self.client.delete(url).send());
        let status = response.status;
        let status_code = status.to_u16();
        let reason = status.canonical_reason();
        let mut body = String::new();
        response.read_to_string(&mut body);
        Ok(AccountKitResponse {
            body: body,
            status_code: status_code,
            canonical_reason: reason,
        })
    }

    pub fn get_user_data<'a>(&self, url: &'a str) -> AccountKitResult {
        let response = try!(self.send_request(url));
        Ok(response)
    }
}


#[cfg(test)]
mod test {
    // Todo
}
