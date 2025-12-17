use crate::errors::ConnectionError;
use http::StatusCode;
use reqwest::{Client, Response};
use std::collections::HashMap;
use thiserror::Error;
use tokio::net::TcpStream;
use url::{ParseError, Url};

#[derive(Error, Debug)]
pub enum VendoError {
    #[error(transparent)]
    ParameterParseFailure(#[from] ParseError),
    #[error(transparent)]
    ServerError(#[from] reqwest::Error),
    #[error("Invalid Http status code {status:?}")]
    HttpStatusError { status: StatusCode },
}

pub struct VendoSocket {
    url: Url,
    client: Client,
}

impl TryFrom<&str> for VendoSocket {
    type Error = VendoError;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            url: Url::parse(url)?,
            client: Client::new(),
        })
    }
}

impl VendoSocket {
    pub async fn request<I>(&self, params: I) -> Result<String, VendoError>
    where
        I: IntoIterator<Item = (String, String)>,
    {
        let param2 = HashMap::from([("from", "8011102"), ("to", "8000105")]);

        let url = reqwest::Url::parse_with_params(self.url.as_str(), param2.iter())?;
        let resp = self.client.get(url).send().await?;
        if resp.status().is_success() {
            Ok(resp.text().await?)
        } else {
            Err(VendoError::HttpStatusError {
                status: resp.status(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[tokio::test]
    async fn test_without_params() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://httpbin.org/")?;
        let params: iter::Empty<(String, String)> = iter::empty();
        socket.request(params).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_with_params() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://v6.db.transport.rest/journeys")?;
        let frankfurt_ibnr = "8000105";
        let berlin_ibnr = "8011102";
        let params = HashMap::from([
            ("from".to_string(), frankfurt_ibnr.to_string()),
            ("to".to_string(), berlin_ibnr.to_string()),
        ]);
        socket.request(params.into_iter()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_bad_status() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://v6.db.transport.rest/journeys")?;
        let params = HashMap::from([("X".to_string(), "Y".to_string())]);
        socket.request(params.into_iter()).await?;
        Ok(())
    }
}
