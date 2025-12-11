use http::StatusCode;
use reqwest::blocking::{Client, Response};
use std::collections::HashMap;
use thiserror::Error;
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
    pub fn request<'a, I>(&self, params: I) -> Result<String, VendoError>
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let url = reqwest::Url::parse_with_params(self.url.as_str(), params)?;
        let resp: Response = self.client.get(url).send()?;
        if resp.status().is_success() {
            Ok(resp.text()?)
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

    #[test]
    fn test_without_params() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://httpbin.org/")?;
        let params: iter::Empty<(&str, &str)> = iter::empty();
        socket.request(params)?;
        Ok(())
    }

    #[test]
    fn test_with_params() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://v6.db.transport.rest/journeys")?;
        let frankfurt_ibnr = "8000105";
        let berlin_ibnr = "8011102";
        let params = HashMap::from([("from", frankfurt_ibnr), ("to", berlin_ibnr)]);
        socket.request(params.into_iter())?;
        Ok(())
    }

    #[test]
    fn test_bad_status() -> Result<(), VendoError> {
        let socket = VendoSocket::try_from("https://v6.db.transport.rest/journeys")?;
        let params = HashMap::from([("X", "Y")]);
        socket.request(params.into_iter())?;
        Ok(())
    }
}
