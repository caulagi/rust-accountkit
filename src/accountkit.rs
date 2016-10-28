// Copyright 2013-2014 The giangnh developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use request::RequestBuilder;
use hyper::method::Method;


#[derive(Debug, PartialEq, Eq)]
pub struct AccountKit<'a> {
    auth: &'a str,
    appsecret_proof: Option<&'a str>,
}

impl<'a> AccountKit<'a> {
    pub fn with_token<S: Into<Option<&'a str>>>(access_token: &'a str,
                                                appsecret_proof: S)
                                                -> AccountKit<'a> {
        AccountKit {
            auth: access_token,
            appsecret_proof: appsecret_proof.into(),
        }
    }

    pub fn authorization_code(&self, authorization_code: &'a str) -> RequestBuilder {
        let mut url = format!("https://graph.accountkit.com/v1.\
                               0/access_token?grant_type=authorization_code&code={}&access_token={}",
                              authorization_code,
                              self.auth);

        append_appsecret_proof(&mut url, self.appsecret_proof);

        RequestBuilder::new(url, Method::Get)

    }

    pub fn validate_token(&self, token: &'a str) -> RequestBuilder {
        let mut url = format!("https://graph.accountkit.com/v1.0/me/?access_token={}",
                              token);
        append_appsecret_proof(&mut url, self.appsecret_proof);
        RequestBuilder::new(url, Method::Get)
    }

    pub fn remove_account(&self, account_id: &'a str, token: &'a str) -> RequestBuilder {
        let mut url = format!("https://graph.accountkit.com/v1.0/{}?access_token={}",
                              account_id,
                              token);
        append_appsecret_proof(&mut url, self.appsecret_proof);
        RequestBuilder::new(url, Method::Delete)
    }

    pub fn retriev_user_data(&self,
                             account_kid_app_id: &'a str,
                             token: &'a str,
                             limit: i32)
                             -> RequestBuilder {
        let limit = limit.to_string();

        let mut url = format!("https://graph.accountkit.com/v1.\
                               0/{}/accounts/?access_token={}&limit={}",
                              account_kid_app_id,
                              token,
                              limit);
        append_appsecret_proof(&mut url, self.appsecret_proof);
        RequestBuilder::new(url, Method::Get)
    }
}

fn append_appsecret_proof<'a>(url: &mut String, value: Option<&'a str>) {
    if value.is_some() {
        url.push_str("&appsecret_proof=");
        url.push_str(value.unwrap());
    }
}


#[test]
fn test_append_appsecret_proof() {
    let mut url = "https://example.com/?access_token=token".to_string();

    append_appsecret_proof(&mut url, Some("proof_key"));
    let expect = "https://example.com/?access_token=token&appsecret_proof=proof_key";
    assert_eq!(expect, url);
}

#[test]
fn test_append_appsecret_proof_empty_value() {
    let mut url = "https://example.com/?access_token=token".to_string();

    append_appsecret_proof(&mut url, None);
    let expect = "https://example.com/?access_token=token";
    assert_eq!(expect, url);
}

#[cfg(test)]
mod test {
    extern crate hyper;
    use AccountKit;
    use self::hyper::method::Method;
    use request::RequestBuilder;

    #[test]
    fn test_with_token() {
        let result = AccountKit::with_token("token", "proof_key");
        let expect = AccountKit {
            auth: "token",
            appsecret_proof: Some("proof_key"),
        };
        assert_eq!(expect, result);
    }

    #[test]
    fn test_authorization_code_empty_appsecret_proof_key() {
        let accountkit = AccountKit::with_token("token", None);
        let result = accountkit.authorization_code("code");
        let expect = RequestBuilder {
            url: "https://graph.accountkit.com/v1.\
                  0/access_token?grant_type=authorization_code&code=code&access_token=token"
                .to_string(),
            method: Method::Get,
        };
        assert_eq!(expect, result);
    }

    #[test]
    fn test_authorization_code_appsecret_proof_key() {
        let accountkit = AccountKit::with_token("token", "appsecret_proof");
        let result = accountkit.authorization_code("code");
        let expect = RequestBuilder {
            url: "https://graph.accountkit.com/v1.\
                  0/access_token?grant_type=authorization_code&code=code&access_token=token&appsecret_proof=appsecret_proof"
                .to_string(),
            method: Method::Get,
        };
        assert_eq!(expect, result);
    }

    #[test]
    fn test_validate_token() {
        let accountkit = AccountKit::with_token("token", "appsecret_proof");
        let result = accountkit.validate_token("token");
        let expect = RequestBuilder {
            url: "https://graph.accountkit.com/v1.\
                  0/me/?access_token=token&appsecret_proof=appsecret_proof"
                .to_string(),
            method: Method::Get,
        };

        assert_eq!(expect, result);
    }

    #[test]
    fn test_remove_account() {
        let accountkit = AccountKit::with_token("token", "appsecret_proof");
        let result = accountkit.remove_account("123", "token");
        let expect = RequestBuilder {
            url: "https://graph.accountkit.com/v1.\
                  0/123?access_token=token&appsecret_proof=appsecret_proof"
                .to_string(),
            method: Method::Delete,
        };

        assert_eq!(expect, result);
    }

    #[test]
    fn test_retriev_user_data() {
        let accountkit = AccountKit::with_token("token", "proof_key");
        let result = accountkit.retriev_user_data("123", "token", 1);
        let expect = RequestBuilder {
            url: "https://graph.accountkit.com/v1.\
                  0/123/accounts/?access_token=token&limit=1&appsecret_proof=proof_key"
                .to_string(),
            method: Method::Get,
        };

        assert_eq!(expect, result);
    }
}
