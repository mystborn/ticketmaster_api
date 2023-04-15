use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

use super::{
    tm_classification::TmClassification,
    tm_dates::TmDates,
    tm_links::{TmLink, TmPaginatedLinks, TmPagination},
    tm_place::TmPlace,
    tm_sales::TmSales,
    tm_structs::{
        TmAccessibility, TmImage, TmLocation, TmOutlet, TmPriceRange, TmProduct, TmPromoter,
        TmSeatmap, TmTicketLimit,
    },
    TmAttraction, TmVenue,
};

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEventLinks {
    #[serde(default, rename = "self")]
    pub current_page: TmLink,

    #[serde(default)]
    pub venues: Vec<TmLink>,

    #[serde(default)]
    pub attractions: Vec<TmLink>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct EventVenuesContainer {
    #[serde(default)]
    pub venues: Vec<TmVenue>,

    #[serde(default)]
    pub attractions: Vec<TmAttraction>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEvent {
    #[serde(default, rename = "_links")]
    pub links: TmEventLinks,

    #[serde(default, rename = "_embedded")]
    pub event_venues: Option<EventVenuesContainer>,

    #[serde(default, rename = "type")]
    pub event_type: String,

    #[serde(default)]
    pub distance: Option<f64>,

    #[serde(default, rename = "units")]
    pub distance_units: Option<String>,

    #[serde(default)]
    pub location: Option<TmLocation>,

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
    pub dates: Option<TmDates>,

    #[serde(default)]
    pub sales: Option<TmSales>,

    #[serde(default)]
    pub info: String,

    #[serde(default, rename = "pleaseNote")]
    pub please_note: String,

    #[serde(default, rename = "priceRanges")]
    pub price_ranges: Vec<TmPriceRange>,

    #[serde(default)]
    pub promoter: Option<TmPromoter>,

    #[serde(default)]
    pub promoters: Vec<TmPromoter>,

    #[serde(default)]
    pub outlets: Vec<TmOutlet>,

    #[serde(default, rename = "productType")]
    pub product_type: String,

    #[serde(default)]
    pub products: Vec<TmProduct>,

    #[serde(default)]
    pub seatmap: Option<TmSeatmap>,

    #[serde(default)]
    pub accessibility: Option<TmAccessibility>,

    #[serde(default, rename = "ticketLimit")]
    pub ticket_limit: Option<TmTicketLimit>,

    #[serde(default)]
    pub classifications: Vec<TmClassification>,

    #[serde(default)]
    pub place: Option<TmPlace>,

    #[serde(default, rename = "externalLinks")]
    pub external_links: Vec<String>,

    #[serde(default)]
    pub test: bool,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default, rename = "localizedAliases")]
    pub localized_aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEventsContainer {
    #[serde(default)]
    pub events: Vec<TmEvent>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEvents {
    #[serde(default, rename = "_links")]
    pub links: TmPaginatedLinks,

    #[serde(default, rename = "_embedded")]
    pub container: TmEventsContainer,

    #[serde(default)]
    pub page: TmPagination,
}
