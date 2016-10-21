// Copyright 2013-2014 The rust-accountkit developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate url;

use std::result::Result;
use self::url::Url;
use error::UtilError;


#[allow(dead_code)]
pub const STATIC_ENDPOINT_URL: &'static str = "https://graph.accountkit.com/v1.0";


pub type UtilResult<T> = Result<T, UtilError>;

#[derive(Debug, PartialEq, Eq)]
pub struct Util {}

impl Util {
    fn parse_endpoint<'a>() -> UtilResult<Url> {
        let url = try!(Url::parse(STATIC_ENDPOINT_URL));
        return Ok(url);
    }
    fn set_access_token<'a>(token: &'a str) -> UtilResult<Url> {
        let mut url = try!(Util::parse_endpoint());
        let input = format!("access_token={}", token);
        url.set_query(Some(input.as_str()));
        return Ok(url);
    }

    fn set_authorization_code<'a>(app_token: &'a str,
                                  authorization_code: &'a str)
                                  -> UtilResult<Url> {
        let mut url = try!(Util::set_access_token(app_token));

        url.query_pairs_mut()
            .append_pair("code", authorization_code)
            .append_pair("grant_type", "authorization_code");

        return Ok(url);
    }
    pub fn url_authorization_code<'a>(app_token: &'a str,
                                      authorization_code: &'a str)
                                      -> UtilResult<Url> {
        let mut url = try!(Util::set_authorization_code(app_token, authorization_code));
        try!(url.path_segments_mut()).extend(&["access_token"]);
        Ok(url)
    }

    pub fn url_validate_token<'a>(token: &'a str) -> UtilResult<Url> {
        let mut url = try!(Util::set_access_token(token));
        try!(url.path_segments_mut()).extend(&["me"]);
        Ok(url)
    }

    pub fn url_remove_account<'a>(account_id: &'a str, token: &'a str) -> UtilResult<Url> {
        let mut url = try!(Util::set_access_token(token));
        try!(url.path_segments_mut()).extend(&[account_id]);
        Ok(url)
    }

    pub fn url_get_user_data<'a>(account_kit_app_id: &'a str,
                                 token: &'a str,
                                 limit: i32)
                                 -> UtilResult<Url> {
        let mut url = try!(Util::set_access_token(token));
        url.query_pairs_mut().append_pair("limit", limit.to_string().as_str());
        try!(url.path_segments_mut()).extend(&[account_kit_app_id, "accounts"]);
        Ok(url)
    }

    pub fn append_app_proof_key<'a>(url: &mut Url, app_proof_key: &'a str) {
        url.query_pairs_mut().append_pair("appsecret_proof", app_proof_key);
    }
}

pub fn get_app_access_token<'a>(facebook_app_id: &'a str,
                                account_kit_app_secret: &'a str)
                                -> String {
    format!("AA|{}|{}", facebook_app_id, account_kit_app_secret)
}


#[cfg(test)]

mod test {
    use util::*;

    #[test]
    fn test_parse_endpoint() {
        let url = Util::parse_endpoint();
        assert!(url.is_ok());
    }

    #[test]
    fn test_set_access_token() {
        let result = Util::set_access_token("app_token");
        assert!(result.is_ok());
    }
    #[test]
    fn test_set_access_token_as_string() {
        let result = Util::set_access_token("app_token").unwrap().into_string();
        let expect = "https://graph.accountkit.com/v1.0?access_token=app_token";
        assert_eq!(expect, result);
    }

    #[test]
    fn test_set_authorization_code() {
        let result = Util::set_authorization_code("app_token", "authorization_code");
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_authorization_code_as_string() {
        let result =
            Util::set_authorization_code("app_token", "authorization_code").unwrap().into_string();
        let expect = "https://graph.accountkit.com/v1.\
                      0?access_token=app_token&code=authorization_code&grant_type=authorization_code";
        assert_eq!(expect, result);
    }

    #[test]
    fn test_url_authorization_code_as_string() {
        let result =
            Util::url_authorization_code("app_token", "authorization_code").unwrap().into_string();
        let expect = "https://graph.accountkit.com/v1.\
                      0/access_token?access_token=app_token&code=authorization_code&grant_type=authorization_code";
        assert_eq!(expect, result);
    }

    #[test]
    fn test_url_validate_token() {
        let result = Util::url_validate_token("app_token");
        assert!(result.is_ok());
    }

    #[test]
    fn test_url_validate_token_as_string() {
        let result = Util::url_validate_token("app_token").unwrap().into_string();
        let expect = "https://graph.accountkit.com/v1.0/me?access_token=app_token";
        assert_eq!(expect, result);
    }

    #[test]
    fn test_url_remove_account() {
        let result = Util::url_remove_account("account_id", "app_token");
        assert!(result.is_ok());
    }

    #[test]
    fn test_url_remove_account_as_string() {
        let result = Util::url_remove_account("account_id", "app_token").unwrap().into_string();
        let expect = "https://graph.accountkit.com/v1.0/account_id?access_token=app_token";
        assert_eq!(expect, result);
    }

    #[test]
    fn test_url_get_user_data() {
        let result = Util::url_get_user_data("account_kit_app_id", "app_token", 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_url_get_user_data_as_string() {
        let result =
            Util::url_get_user_data("account_kit_app_id", "app_token", 10).unwrap().into_string();
        let expect = "https://graph.accountkit.com/v1.\
                      0/account_kit_app_id/accounts?access_token=app_token&limit=10";
        assert_eq!(expect, result);
    }

    #[test]
    fn test_append_app_proof_key() {
        let mut url = Util::url_validate_token("app_token").unwrap();
        Util::append_app_proof_key(&mut url, "app_proof_key");
        let expect = "https://graph.accountkit.com/v1.\
                      0/me?access_token=app_token&appsecret_proof=app_proof_key";
        assert_eq!(expect, url.into_string());
    }

    #[test]
    fn test_get_app_access_token() {
        let result = get_app_access_token("facebook_app_id", "account_kit_app_secret");
        let expect = "AA|facebook_app_id|account_kit_app_secret";
        assert_eq!(expect, result);
    }
}
