use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmLink {
    #[serde(default)]
    href: String,

    #[serde(default)]
    templated: bool
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmSimpleLinks {
    #[serde(default, rename="self")]
    current_page: TmLink
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmPaginatedLinks {
    #[serde(default, rename="self")]
    current_page: TmLink,

    #[serde(default)]
    next: Option<TmLink>,

    #[serde(default)]
    prev: Option<TmLink>
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmPagination {
    #[serde(default)]
    size: u32,

    #[serde(default, rename = "totalElements")]
    total_elements: u32,

    #[serde(default, rename = "totalPages")]
    total_pages: u32,

    #[serde(default)]
    number: u32
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmTwitter {

    #[serde(default)]
    handle: String,

    #[serde(default)]
    hashtags: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmSocial {

    #[serde(default)]
    twitter: TmTwitter
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmExternalUrl {
    #[serde(default)]
    url: String
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmExternalLinks {
    #[serde(default, flatten)]
    links: HashMap<String, Vec<TmExternalUrl>>
}