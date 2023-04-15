use std::collections::HashMap;

use either::Either;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use super::TmSimpleLinks;

pub fn default_as_true() -> bool {
    true
}

pub fn invalid_value_type() -> i32 {
    -1
}

pub fn default_either<L, R>() -> Either<L, R>
where
    L: Default,
{
    Either::Left(L::default())
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPriceRange {
    #[serde(default, rename = "type")]
    pub price_type: Option<String>,

    #[serde(default)]
    pub currency: String,

    #[serde(default)]
    pub min: f32,

    #[serde(default)]
    pub max: f32,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPromoter {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmOutlet {
    #[serde(default)]
    pub url: String,

    #[serde(default, rename = "type")]
    pub outlet_type: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmProduct {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub url: String,

    #[serde(default, rename = "type")]
    pub product_type: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSeatmap {
    #[serde(default, rename = "staticUrl")]
    pub static_url: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAccessibility {
    #[serde(default)]
    pub info: String,

    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmTicketLimit {
    #[serde(default)]
    pub infos: Value,

    #[serde(default)]
    pub info: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocation {
    #[serde(default = "default_either", with = "either::serde_untagged")]
    pub latitude: Either<f64, String>,

    #[serde(default = "default_either", with = "either::serde_untagged")]
    pub longitude: Either<f64, String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmImage {
    #[serde(default)]
    pub url: String,

    #[serde(default)]
    pub ratio: String,

    #[serde(default)]
    pub width: u32,

    #[serde(default)]
    pub height: u32,

    #[serde(default)]
    pub fallback: bool,

    #[serde(default)]
    pub attribution: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmImages {
    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default, rename = "type")]
    pub images_type: String,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub images: Vec<TmImage>,
}
