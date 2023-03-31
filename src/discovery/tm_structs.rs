use std::collections::HashMap;

use either::Either;
use serde::{Serialize, Deserialize};
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
    where L: Default 
{
    Either::Left(L::default())
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPriceRange {
    #[serde(default, rename="type")]
    price_type: Option<String>,

    #[serde(default)]
    currency: String,

    #[serde(default)]
    min: f32,

    #[serde(default)]
    max: f32
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPromoter {
    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmOutlet {
    #[serde(default)]
    url: String,

    #[serde(default, rename="type")]
    outlet_type: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmProduct {
    #[serde(default)]
    name: String,

    #[serde(default)]
    id: String,

    #[serde(default)]
    url: String,

    #[serde(default, rename="type")]
    product_type: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSeatmap {
    #[serde(default, rename="staticUrl")]
    static_url: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAccessibility {
    #[serde(default)]
    info: String,

    #[serde(default, flatten)]
    extra: HashMap<String, Value>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmTicketLimit {
    #[serde(default)]
    infos: Value,

    #[serde(default)]
    info: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocation {

    #[serde(default="default_either", with="either::serde_untagged")]
    latitude: Either<f64, String>,

    #[serde(default="default_either", with="either::serde_untagged")]
    longitude: Either<f64, String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmImage {
    #[serde(default)]
    url: String,

    #[serde(default)]
    ratio: String,

    #[serde(default)]
    width: u32,

    #[serde(default)]
    height: u32,

    #[serde(default)]
    fallback: bool,

    #[serde(default)]
    attribution: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmImages {
    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default, rename="type")]
    images_type: String,

    #[serde(default)]
    id: String,

    #[serde(default)]
    images: Vec<TmImage>,
}

