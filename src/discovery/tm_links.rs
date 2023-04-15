use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmLink {
    #[serde(default)]
    pub href: String,

    #[serde(default)]
    pub templated: bool,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmSimpleLinks {
    #[serde(default, rename = "self")]
    pub current_page: TmLink,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmPaginatedLinks {
    #[serde(default, rename = "self")]
    pub current_page: TmLink,

    #[serde(default)]
    pub next: Option<TmLink>,

    #[serde(default)]
    pub prev: Option<TmLink>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmPagination {
    #[serde(default)]
    pub size: u32,

    #[serde(default, rename = "totalElements")]
    pub total_elements: u32,

    #[serde(default, rename = "totalPages")]
    pub total_pages: u32,

    #[serde(default)]
    pub number: u32,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmTwitter {
    #[serde(default)]
    pub handle: String,

    #[serde(default)]
    pub hashtags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmSocial {
    #[serde(default)]
    pub twitter: TmTwitter,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmExternalUrl {
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmExternalLinks {
    #[serde(default, flatten)]
    pub links: HashMap<String, Vec<TmExternalUrl>>,
}
