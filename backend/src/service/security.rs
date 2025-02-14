use crate::{
    prelude::*,
    security::jwt_util::{TokenFactory, TokenPair},
};

pub fn refresh(token: String) -> Result<TokenPair> {
    let factory = TokenFactory::from_env()?;
    let subject = factory.subject(&token)?;
    factory.generate_token(subject)
}
