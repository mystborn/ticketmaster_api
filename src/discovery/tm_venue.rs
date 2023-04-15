use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use super::{
    TmAddress, TmClassification, TmCountry, TmExternalLinks, TmImage, TmLocation, TmPaginatedLinks,
    TmPagination, TmPlaceNameValue, TmSimpleLinks, TmSocial, TmState,
};

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmMarket {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmDma {
    #[serde(default)]
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmBoxOffice {
    #[serde(default, rename = "phoneNumberDetail")]
    pub phone_number_detail: String,

    #[serde(default, rename = "openHoursDetail")]
    pub open_hours_detail: String,

    #[serde(default, rename = "acceptedPaymentDetail")]
    pub accepted_payment_detail: String,

    #[serde(default, rename = "willCallDetail")]
    pub will_call_detail: String,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmVenueInfo {
    #[serde(default, rename = "generalRule")]
    pub general_rule: Option<String>,

    #[serde(default, rename = "childRule")]
    pub child_rule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmVenueAda {
    #[serde(default, rename = "adaPhones")]
    pub ada_phones: Option<String>,

    #[serde(default, rename = "adaCustomCopy")]
    pub ada_custom_copy: Option<String>,

    #[serde(default, rename = "adaHours")]
    pub ada_hours: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmVenue {
    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default, rename = "type")]
    pub venue_type: String,

    #[serde(default)]
    pub distance: f64,

    #[serde(default, rename = "units")]
    pub distance_units: String,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub locale: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub address: TmAddress,

    #[serde(default)]
    pub city: TmPlaceNameValue,

    #[serde(default, rename = "additionalInfo")]
    pub additional_info: Option<String>,

    #[serde(default)]
    pub state: TmState,

    #[serde(default)]
    pub country: TmCountry,

    #[serde(default)]
    pub url: String,

    #[serde(default, rename = "postalCode")]
    pub postal_code: String,

    #[serde(default)]
    pub location: TmLocation,

    #[serde(default)]
    pub timezone: String,

    #[serde(default)]
    pub currency: Option<String>,

    #[serde(default)]
    pub markets: Vec<TmMarket>,

    #[serde(default)]
    pub images: Vec<TmImage>,

    #[serde(default)]
    pub dmas: Vec<TmDma>,

    #[serde(default)]
    pub social: Option<TmSocial>,

    #[serde(default, rename = "boxOfficeInfo")]
    pub box_office_info: Option<TmBoxOffice>,

    #[serde(default, rename = "parkingDetail")]
    pub parking_detail: Option<String>,

    #[serde(default, rename = "accessibleSeatingDetail")]
    pub accessible_seating_detail: Option<String>,

    #[serde(default, rename = "generalInfo")]
    pub general_info: Option<TmVenueInfo>,

    #[serde(default, rename = "externalLinks")]
    pub external_links: Option<TmExternalLinks>,

    #[serde(default)]
    pub test: bool,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default, rename = "localizedAliases")]
    pub localized_aliases: Value,

    #[serde(default, rename = "upcomingEvents")]
    pub upcoming_events: HashMap<String, u32>,

    #[serde(default)]
    pub ada: Option<TmVenueAda>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAttraction {
    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default, rename = "type")]
    pub attraction_type: String,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub locale: Option<String>,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default, rename = "additionalInfo")]
    pub additional_info: Option<String>,

    #[serde(default)]
    pub url: String,

    #[serde(default)]
    pub images: Vec<TmImage>,

    #[serde(default)]
    pub classifications: Vec<TmClassification>,

    #[serde(default, rename = "externalLinks")]
    pub external_links: Option<TmExternalLinks>,

    #[serde(default)]
    pub test: bool,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default, rename = "localizedAliases")]
    pub localized_aliases: Value,

    #[serde(default, rename = "upcomingEvents")]
    pub upcoming_events: HashMap<String, u32>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAttractionsContainer {
    #[serde(default)]
    pub attractions: Vec<TmAttraction>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAttractions {
    #[serde(default, rename = "_links")]
    pub links: TmPaginatedLinks,

    #[serde(default, rename = "_embedded")]
    pub container: TmAttractionsContainer,

    #[serde(default)]
    pub page: TmPagination,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmVenuesContainer {
    #[serde(default)]
    pub venues: Vec<TmVenue>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmVenuesSearch {
    #[serde(default, rename = "_links")]
    pub links: TmPaginatedLinks,

    #[serde(default, rename = "_embedded")]
    pub container: TmVenuesContainer,

    #[serde(default)]
    pub page: TmPagination,
}
