//! Module that contains the main [Freecurrencyapi] struct

use std::sync::Arc;
use reqwest::Client;
use crate::error::FreecurrencyapiError;
use crate::{error, models, utils};
use crate::utils::baseline::construct_base_url;

/// Settings struct that contains the api key
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
}

/// The main struct of the crate giving access to the freecurrencyapi.
/// Create a new instance of the struct with your api key as parameter.
#[derive(Debug, Clone)]
pub struct Freecurrencyapi {
    client: Client,
    settings: Arc<Settings>,
}

impl<'a> Freecurrencyapi {
    /// Creates a new instance of the Freecurrencyapi struct by passing your api key as
    /// function parameter.
    pub fn new(api_key: &'a str) -> Result<Self, FreecurrencyapiError> {
        let settings = std::sync::Arc::new(Settings {
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(None, &settings)?;
        Ok(Self { client, settings })
    }

    pub async fn status(
        &self,
    ) -> Result<models::DetailsResponse, error::FreecurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("status"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::FreecurrencyapiError::ResponseParsingError { body: res_body })
    }

    pub async fn currencies(
        &self,
    ) -> Result<models::DetailsResponse, error::FreecurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("currencies"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::FreecurrencyapiError::ResponseParsingError { body: res_body })
    }

    pub async fn latest(
        &self,
        base_currency: &'a str,
        currencies: &'a str,
    ) -> Result<models::DetailsResponse, error::FreecurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("latest"))?;
        url.query_pairs_mut()
            .append_pair("base_currency", base_currency)
            .append_pair("currencies", currencies);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::FreecurrencyapiError::ResponseParsingError { body: res_body })
    }

    pub async fn historical(
        &self,
        base_currency: &'a str,
        date: &'a str,
        currencies: &'a str,
    ) -> Result<models::DetailsResponse, error::FreecurrencyapiError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("historical"))?;
        url.query_pairs_mut()
            .append_pair("base_currency", base_currency)
            .append_pair("date", date)
            .append_pair("currencies", currencies);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::FreecurrencyapiError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::FreecurrencyapiError::ResponseParsingError { body: res_body })
    }

}
