use std::collections::HashMap;

use reqwest::Client;
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Error, Debug)]
pub enum VendoError {
    #[error(transparent)]
    ParameterParseFailure(#[from] ParseError),
    #[error(transparent)]
    ServerError(#[from] reqwest::Error),
}

pub struct VendoSocket {
    url: Url,
    client: Client,
}

impl TryFrom<&str> for VendoSocket {
    type Error = ParseError;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            url: Url::parse(url)?,
            client: reqwest::Client::new(),
        })
    }
}

impl VendoSocket {
    async fn request<'a, I>(&self, params: I) -> Result<String, VendoError>
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let url = reqwest::Url::parse_with_params(self.url.as_str(), params)?;
        let body = reqwest::get(url).await?.text().await?;
        Ok(body)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn test_without_params() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://httpbin.org/")?;
        let params = iter::empty();
        assert!(socket.request(params).await);
        Ok(())
    }
}
