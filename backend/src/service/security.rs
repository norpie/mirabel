use crate::{
    prelude::*,
    security::jwt_util::{TokenFactory, TokenPair},
};

pub fn refresh(subject: String) -> Result<TokenPair> {
    let factory = TokenFactory::from_env()?;
    factory.generate_token(subject)
}
