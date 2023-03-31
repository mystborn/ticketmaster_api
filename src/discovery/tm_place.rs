use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;

use super::tm_structs::TmLocation;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPlaceNameValue {

    #[serde(default)]
    name: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAddress {

    #[serde(default)]
    line1: String,

    #[serde(default)]
    line2: Option<String>,

    #[serde(default)]
    line3: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmState {

    #[serde(default, rename="stateCode")]
    state_code: Option<String>,

    #[serde(default)]
    name: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmCountry {

    #[serde(default, rename="countryCode")]
    country_code: String,

    #[serde(default)]
    name: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPlace {

    #[serde(default)]
    area: Option<TmPlaceNameValue>,

    #[serde(default)]
    address: TmAddress,

    #[serde(default)]
    city: TmPlaceNameValue,

    #[serde(default)]
    state: TmState,

    #[serde(default)]
    country: TmCountry,

    #[serde(default, rename="postalCode")]
    postal_code: String,

    #[serde(default)]
    location: Option<TmLocation>,

    #[serde(default)]
    name: Option<String>,
}