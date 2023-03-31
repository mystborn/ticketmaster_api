use std::{error::Error, time::Duration};

use either::Either;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;

use super::{TmEvent, TmEvents, TmImages, EventSearchQuery, DetailsQuery, AttractionSearchQuery, TmAttractions, TmAttraction, ClassificationSearchQuery, TmClassifications};

use serde_qs;

pub struct TicketmasterDiscoveryClient {
    api_key: String,
    client: Client
}

const API_PREFIX: &'static str = "https://app.ticketmaster.com/discovery/v2/";
const EVENT_SEARCH: &'static str = "events.json?";
const ATTRACTION_SEARCH: &'static str = "attractions.json?";
const CLASSIFICATION_SEARCH: &'static str = "classifications.json?";

impl TicketmasterDiscoveryClient {
    pub fn new(api_key: String) -> TicketmasterDiscoveryClient {
        TicketmasterDiscoveryClient {
            api_key,
            client: ClientBuilder::new()
                .timeout(Duration::new(10, 0))
                .build()
                .unwrap()
        }
    }

    fn event_search_url(&self, search_query: Either<&EventSearchQuery, &str>) -> Result<String, Box<dyn Error>> {
        let mut url = API_PREFIX.to_string();
        url.push_str(EVENT_SEARCH);

        match search_query {
            Either::Left(obj) => url.push_str(serde_qs::to_string(obj)?.as_str()),
            Either::Right(text) => url.push_str(text)
        }

        self.append_api_key(&mut url);

        Ok(url)
    }

    pub async fn event_search(&self, search_query: Either<&EventSearchQuery, &str>) -> Result<TmEvents, Box<dyn Error>> {
        let url = self.event_search_url(search_query)?;

        self.get_object(url).await
    }

    pub async fn event_search_as_json(&self, search_query: Either<&EventSearchQuery, &str>) -> Result<String, Box<dyn Error>> {
        let url = self.event_search_url(search_query)?;

        self.get_text(url).await
    }

    fn event_details_url(&self, event_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<String, Box<dyn Error>> {
        let mut url = API_PREFIX.to_string();
        url.push_str(event_id);
        url.push_str(".json?");

        if let Some(inner) = search_query {
            match inner {
                Either::Left(obj) => url.push_str(serde_qs::to_string(obj)?.as_str()),
                Either::Right(text) => url.push_str(text)
            }
        }

        self.append_api_key(&mut url);

        Ok(url)
    }

    pub async fn event_details(&self, event_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<TmEvent, Box<dyn Error>> {
        let url = self.event_details_url(event_id, search_query)?;

        self.get_object(url).await
    }

    pub async fn event_details_as_json(&self, event_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<String, Box<dyn Error>> {
        let url = self.event_details_url(event_id, search_query)?;

        self.get_text(url).await
    }

    fn event_images_url(&self, event_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<String, Box<dyn Error>> {
        let mut url = API_PREFIX.to_string();
        url.push_str(event_id);
        url.push_str("/images.json?");

        if let Some(inner) = search_query {
            match inner {
                Either::Left(obj) => url.push_str(serde_qs::to_string(obj)?.as_str()),
                Either::Right(text) => url.push_str(text)
            }
        }

        self.append_api_key(&mut url);

        Ok(url)
    }

    pub async fn event_images(&self, event_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<TmImages, Box<dyn Error>> {
        let url = self.event_images_url(event_id, search_query)?;

        self.get_object(url).await
    }

    pub async fn event_images_as_json(&self, event_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<String, Box<dyn Error>> {
        let url = self.event_images_url(event_id, search_query)?;

        self.get_text(url).await
    }

    fn attraction_search_url(&self, search_query: Either<&AttractionSearchQuery, &str>) -> Result<String, Box<dyn Error>> {
        let mut url = API_PREFIX.to_string();
        url.push_str(ATTRACTION_SEARCH);
        match search_query {
            Either::Left(obj) => url.push_str(serde_qs::to_string(obj)?.as_str()),
            Either::Right(text) => url.push_str(text),
        }
        self.append_api_key(&mut url);

        Ok(url)
    }

    pub async fn attraction_search(&self, search_query: Either<&AttractionSearchQuery, &str>) -> Result<TmAttractions, Box<dyn Error>> {
        let url = self.attraction_search_url(search_query)?;

        self.get_object(url).await
    }

    pub async fn attraction_search_to_json(&self, search_query: Either<&AttractionSearchQuery, &str>) -> Result<String, Box<dyn Error>> {
        let url = self.attraction_search_url(search_query)?;

        self.get_text(url).await
    }

    fn attraction_details_url(&self, attraction_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<String, Box<dyn Error>> {
        let mut url = API_PREFIX.to_string();
        url.push_str("attractions/");
        url.push_str(attraction_id);
        url.push_str(".json?");
        
        if let Some(inner) = search_query {
            match inner {
                Either::Left(obj) => url.push_str(serde_qs::to_string(obj)?.as_str()),
                Either::Right(text) => url.push_str(text)
            }
        }

        self.append_api_key(&mut url);

        Ok(url)
    }

    pub async fn attraction_details(&self, attraction_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<TmAttraction, Box<dyn Error>> {
        let url = self.attraction_details_url(attraction_id, search_query)?;

        self.get_object(url).await
    }

    pub async fn attraction_details_as_json(&self, attraction_id: &str, search_query: Option<Either<&DetailsQuery, &str>>) -> Result<String, Box<dyn Error>> {
        let url = self.attraction_details_url(attraction_id, search_query)?;

        self.get_text(url).await
    }

    fn classification_search_url(&self, search_query: Either<&ClassificationSearchQuery, &str>) -> Result<String, Box<dyn Error>> {
        let mut url = API_PREFIX.to_string();
        url.push_str(CLASSIFICATION_SEARCH);

        match search_query {
            Either::Left(obj) => url.push_str(serde_qs::to_string(obj)?.as_str()),
            Either::Right(text) => url.push_str(text)
        }

        self.append_api_key(&mut url);

        Ok(url)
    }

    pub async fn classification_search(&self, search_query: Either<&ClassificationSearchQuery, &str>) -> Result<TmClassifications, Box<dyn Error>> {
        let url = self.classification_search_url(search_query)?;

        self.get_object(url).await
    }

    pub async fn classification_search_as_json(&self, search_query: Either<&ClassificationSearchQuery, &str>) -> Result<String, Box<dyn Error>> {
        let url = self.classification_search_url(search_query)?;

        self.get_text(url).await
    }

    async fn get_object<T>(&self, url: String) -> Result<T, Box<dyn Error>>
        where for<'a> T : Deserialize<'a>
    {
        let obj = self.client
            .get(url)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(obj)
    }

    async fn get_text(&self, url: String) -> Result<String, Box<dyn Error>> {
        let text = self.client
            .get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(text)
    }

    pub fn test_query_serialize(&self, search_query: &EventSearchQuery) -> Result<String, Box<dyn Error>> {
        Ok(serde_qs::to_string(search_query)?)
    }

    pub fn append_api_key(&self, url: &mut String) {
        if !url.ends_with("?") && !url.ends_with("/") && !url.ends_with("&") {
            url.push_str("&");
        }

        url.push_str(format!("apikey={}", self.api_key).as_str());
    }
}