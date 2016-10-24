use std::result::Result;
use std::io::Read;

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
    body: String,
    status_code: u16,
    canonical_reason: Option<&'static str>,
}

pub trait DoRequest: BaseRequest {
    fn retriev_json(&self) -> Result<AccountKitResponse, AccountKitError> {
        let url = try!(Url::parse(self.url()));
        let fresh_net = try!(request::Request::new(self.method(), url));
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
