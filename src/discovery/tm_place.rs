use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

use super::tm_structs::TmLocation;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPlaceNameValue {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAddress {
    #[serde(default)]
    pub line1: String,

    #[serde(default)]
    pub line2: Option<String>,

    #[serde(default)]
    pub line3: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmState {
    #[serde(default, rename = "stateCode")]
    pub state_code: Option<String>,

    #[serde(default)]
    pub name: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmCountry {
    #[serde(default, rename = "countryCode")]
    pub country_code: String,

    #[serde(default)]
    pub name: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPlace {
    #[serde(default)]
    pub area: Option<TmPlaceNameValue>,

    #[serde(default)]
    pub address: TmAddress,

    #[serde(default)]
    pub city: TmPlaceNameValue,

    #[serde(default)]
    pub state: TmState,

    #[serde(default)]
    pub country: TmCountry,

    #[serde(default, rename = "postalCode")]
    pub postal_code: String,

    #[serde(default)]
    pub location: Option<TmLocation>,

    #[serde(default)]
    pub name: Option<String>,
}
