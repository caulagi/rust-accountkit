#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub facebook_app_id: String,
    pub accountkit_app_secret: String,
}

impl Token {
    pub fn new<S>(facebook_app_id: S, accountkit_app_secret: S) -> Token
        where S: Into<String>
    {
        Token {
            facebook_app_id: facebook_app_id.into(),
            accountkit_app_secret: accountkit_app_secret.into(),
        }
    }

    pub fn get_access_token(&self) -> String {
        format!("AA|{}|{}", self.facebook_app_id, self.accountkit_app_secret)
    }
}

#[cfg(test)]

mod test {
    use Token;
    #[test]
    fn test_construct_new_token() {
        let result = Token::new("app_id", "accountkit_app_secret");

        assert_eq!(Token {
                       facebook_app_id: "app_id".to_string(),
                       accountkit_app_secret: "accountkit_app_secret".to_string(),
                   },
                   result);
    }

    #[test]
    fn test_get_access_token() {
        let token = Token::new("app_id", "accountkit_app_secret");
        let result = token.get_access_token();

        assert_eq!("AA|app_id|accountkit_app_secret", result);
    }
}
