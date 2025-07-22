use crate::prelude::*;

use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone)]
pub struct TokenFactory {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl TokenFactory {
    pub fn generate_token(&self, sub: String) -> Result<TokenPair> {
        let access = Claims::new(sub.clone(), TokenType::Access);
        let refresh = Claims::new(sub, TokenType::Refresh);

        let access_jwt = self.generate_token_from_claims(access)?;
        let refresh_jwt = self.generate_token_from_claims(refresh)?;

        TokenPair::new(access_jwt, refresh_jwt)
    }

    fn generate_token_from_claims(&self, claims: Claims) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &self.encoding,
        )?)
    }

    pub fn subject(&self, token: &str) -> Result<String> {
        let token = jsonwebtoken::decode::<Claims>(
            token,
            &self.decoding,
            &jsonwebtoken::Validation::default(),
        )?;
        Ok(token.claims.sub)
    }

    pub fn from_env() -> Result<Self> {
        let secret = std::env::var("JWT_SECRET")?;
        Ok(secret.into())
    }
}

impl From<String> for TokenFactory {
    fn from(secret: String) -> Self {
        Self::from(secret.as_str())
    }
}

impl From<&str> for TokenFactory {
    fn from(secret: &str) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    tt: TokenType,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
}

impl Claims {
    fn new(sub: String, token_type: TokenType) -> Self {
        Self {
            exp: token_type.valid_until(),
            tt: token_type,
            iat: Utc::now().timestamp() as usize,
            iss: "Mirabel Backend".into(),
            sub,
        }
    }
}

impl Claims {
    fn is_valid(&self) -> bool {
        self.exp > Utc::now().timestamp() as usize
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum TokenType {
    Access,
    Refresh,
}

impl TokenType {
    fn duration(&self) -> Duration {
        match self {
            TokenType::Access => Duration::days(1),
            TokenType::Refresh => Duration::days(30),
        }
    }

    fn valid_until(&self) -> usize {
        (Utc::now() + self.duration()).timestamp() as usize
    }
}

pub struct TokenPair(String, String);

impl TokenPair {
    fn new(access: String, refresh: String) -> Result<Self> {
        Ok(Self(access, refresh))
    }

    pub fn access(&self) -> &str {
        &self.0
    }

    pub fn refresh(&self) -> &str {
        &self.1
    }
}

#[cfg(test)]
mod tests {
use super::TokenFactory;
use super::TokenPair;
    use crate::prelude::*;

    fn make_test_factory() -> TokenFactory {
        "test".into()
    }

    fn generate_test_token() -> Result<TokenPair> {
        make_test_factory().generate_token("test".into())
    }

    #[test]
    fn test_generate_token() {
        generate_test_token().unwrap();
    }

    #[test]
    fn test_subject() {
        let factory = make_test_factory();
        let token = generate_test_token().unwrap();
        let subject = factory.subject(token.access()).unwrap();
        assert_eq!(subject, "test");
    }

    #[test]
    fn test_token_pair() {
        let token = generate_test_token().unwrap();
        assert_eq!(token.access(), token.access());
        assert_eq!(token.refresh(), token.refresh());
        assert_ne!(token.access(), token.refresh());
    }
}
