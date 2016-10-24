use std::fmt;
use std::result::Result;

use hyper::method::Method;

use error::AccountKitError;

pub use self::dorequest::{AccountKitResponse, BaseRequest, DoRequest};

pub mod dorequest;

#[derive(Debug, PartialEq, Eq)]
pub struct RequestBuilder {
    pub url: String,
    pub method: Method,
}

impl fmt::Display for RequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: {}\nurl: {} \n", self.method(), self.url())
    }
}


impl BaseRequest for RequestBuilder {
    fn url(&self) -> &str {
        &self.url[..]
    }

    fn method(&self) -> Method {
        self.method.clone()
    }
}


impl RequestBuilder {
    pub fn new<S: Into<String>>(url: S, method: Method) -> RequestBuilder {
        RequestBuilder {
            url: url.into(),
            method: method,
        }

    }
}

impl DoRequest for RequestBuilder {
    fn retriev(&self) -> Result<AccountKitResponse, AccountKitError> {
        let result = try!(self.retriev_json());
        Ok(result)
    }
}



#[cfg(test)]

mod test {
    extern crate hyper;
    use RequestBuilder;
    use self::hyper::method::Method;
    use request::dorequest::{BaseRequest, DoRequest};

    #[test]
    fn test_new_request_builder() {
        let result = RequestBuilder::new("https://example.com", Method::Get);
        let expect = RequestBuilder {
            url: "https://example.com".to_string(),
            method: Method::Get,
        };
        assert_eq!(expect, result);
    }

    #[test]
    fn test_get_url() {
        let request_builder = RequestBuilder::new("https://example.com", Method::Get);
        let result = request_builder.url();
        let expect = "https://example.com";
        assert_eq!(expect, result);
    }
    #[test]
    fn test_get_method() {
        let request_builder = RequestBuilder::new("https://example.com", Method::Delete);
        let result = request_builder.method();
        let expect = Method::Delete;
        assert_eq!(expect, result);
    }

    #[test]
    fn test_retriev_data() {
        let request_builder = RequestBuilder::new("https://google.com.vn", Method::Get);
        let response = request_builder.retriev();

        assert!(response.is_ok());
    }

    #[test]
    fn test_parse_invalid_url() {
        let request_builder = RequestBuilder::new("https://example .com", Method::Get);
        let response = request_builder.retriev();
        let expect = "invalid international domain name";
        match response {
            Ok(_) => {}
            Err(err) => assert_eq!(expect, err.to_string()),
        }
    }
}
