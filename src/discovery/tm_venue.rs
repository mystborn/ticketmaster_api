use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;
use serde_json::Value;

use super::{TmAddress, TmPlaceNameValue, TmState, TmCountry, TmLocation, TmImage, TmExternalLinks, TmSimpleLinks, TmSocial, TmPaginatedLinks, TmPagination, TmClassification};

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmMarket {

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmDma {

    #[serde(default)]
    id: i32
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmBoxOffice {

    #[serde(default, rename="phoneNumberDetail")]
    phone_number_detail: String,

    #[serde(default, rename="openHoursDetail")]
    open_hours_detail: String,

    #[serde(default, rename="acceptedPaymentDetail")]
    accepted_payment_detail: String,

    #[serde(default, rename="willCallDetail")]
    will_call_detail: String
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmVenueInfo {

    #[serde(default, rename="generalRule")]
    general_rule: Option<String>,

    #[serde(default, rename="childRule")]
    child_rule: Option<String>
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmVenueAda {

    #[serde(default, rename="adaPhones")]
    ada_phones: Option<String>,

    #[serde(default, rename="adaCustomCopy")]
    ada_custom_copy: Option<String>,

    #[serde(default, rename="adaHours")]
    ada_hours: Option<String>
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmVenue {
    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default, rename="type")]
    venue_type: String,

    #[serde(default)]
    distance: f64,

    #[serde(default, rename="units")]
    distance_units: String,

    #[serde(default)]
    id: String,

    #[serde(default)]
    locale: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    address: TmAddress,

    #[serde(default)]
    city: TmPlaceNameValue,

    #[serde(default, rename="additionalInfo")]
    additional_info: Option<String>,

    #[serde(default)]
    state: TmState,

    #[serde(default)]
    country: TmCountry,

    #[serde(default)]
    url: String,

    #[serde(default, rename="postalCode")]
    postal_code: String,

    #[serde(default)]
    location: TmLocation,

    #[serde(default)]
    timezone: String,

    #[serde(default)]
    currency: Option<String>,

    #[serde(default)]
    markets: Vec<TmMarket>,

    #[serde(default)]
    images: Vec<TmImage>,

    #[serde(default)]
    dmas: Vec<TmDma>,

    #[serde(default)]
    social: Option<TmSocial>,

    #[serde(default, rename="boxOfficeInfo")]
    box_office_info: Option<TmBoxOffice>,

    #[serde(default, rename="parkingDetail")]
    parking_detail: Option<String>,

    #[serde(default, rename="accessibleSeatingDetail")]
    accessible_seating_detail: Option<String>,

    #[serde(default, rename="generalInfo")]
    general_info: Option<TmVenueInfo>,

    #[serde(default, rename="externalLinks")]
    external_links: Option<TmExternalLinks>,

    #[serde(default)]
    test: bool,

    #[serde(default)]
    aliases: Vec<String>,

    #[serde(default, rename="localizedAliases")]
    localized_aliases: Value,

    #[serde(default, rename="upcomingEvents")]
    upcoming_events: HashMap<String, u32>,

    #[serde(default)]
    ada: Option<TmVenueAda>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAttraction {
    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default, rename="type")]
    attraction_type: String,

    #[serde(default)]
    id: String,

    #[serde(default)]
    locale: Option<String>,

    #[serde(default)]
    name: String,

    #[serde(default)]
    description: Option<String>,

    #[serde(default, rename="additionalInfo")]
    additional_info: Option<String>,

    #[serde(default)]
    url: String,

    #[serde(default)]
    images: Vec<TmImage>,

    #[serde(default)]
    classifications: Vec<TmClassification>,

    #[serde(default, rename="externalLinks")]
    external_links: Option<TmExternalLinks>,

    #[serde(default)]
    test: bool,

    #[serde(default)]
    aliases: Vec<String>,

    #[serde(default, rename="localizedAliases")]
    localized_aliases: Value,

    #[serde(default, rename="upcomingEvents")]
    upcoming_events: HashMap<String, u32>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAttractionsContainer {
    #[serde(default)]
    attractions: Vec<TmAttraction>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmAttractions {
    #[serde(default, rename="_links")]
    links: TmPaginatedLinks,

    #[serde(default, rename="_embedded")]
    container: TmAttractionsContainer,

    #[serde(default)]
    page: TmPagination
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmVenuesContainer {
    #[serde(default)]
    venues: Vec<TmVenue>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmVenuesSearch {
    #[serde(default, rename="_links")]
    links: TmPaginatedLinks,

    #[serde(default, rename="_embedded")]
    container: TmVenuesContainer,

    #[serde(default)]
    page: TmPagination
}