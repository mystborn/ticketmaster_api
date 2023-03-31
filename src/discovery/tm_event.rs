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
    }, TmVenue, TmAttraction,
};

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEventLinks {
    #[serde(default, rename = "self")]
    current_page: TmLink,

    #[serde(default)]
    venues: Vec<TmLink>,

    #[serde(default)]
    attractions: Vec<TmLink>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct EventVenuesContainer {
    #[serde(default)]
    venues: Vec<TmVenue>,

    #[serde(default)]
    attractions: Vec<TmAttraction>
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEvent {
    #[serde(default, rename="_links")]
    links: TmEventLinks,

    #[serde(default, rename="_embedded")]
    event_venues: Option<EventVenuesContainer>,

    #[serde(default, rename="type")]
    event_type: String,

    #[serde(default)]
    distance: Option<f64>,

    #[serde(default, rename="units")]
    distance_units: Option<String>,

    #[serde(default)]
    location: Option<TmLocation>,

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
    dates: Option<TmDates>,

    #[serde(default)]
    sales: Option<TmSales>,

    #[serde(default)]
    info: String,

    #[serde(default, rename="pleaseNote")]
    please_note: String,

    #[serde(default, rename="priceRanges")]
    price_ranges: Vec<TmPriceRange>,

    #[serde(default)]
    promoter: Option<TmPromoter>,

    #[serde(default)]
    promoters: Vec<TmPromoter>,

    #[serde(default)]
    outlets: Vec<TmOutlet>,

    #[serde(default, rename="productType")]
    product_type: String,

    #[serde(default)]
    products: Vec<TmProduct>,

    #[serde(default)]
    seatmap: Option<TmSeatmap>,

    #[serde(default)]
    accessibility: Option<TmAccessibility>,

    #[serde(default, rename="ticketLimit")]
    ticket_limit: Option<TmTicketLimit>,

    #[serde(default)]
    classifications: Vec<TmClassification>,

    #[serde(default)]
    place: Option<TmPlace>,

    #[serde(default, rename="externalLinks")]
    external_links: Vec<String>,

    #[serde(default)]
    test: bool,

    #[serde(default)]
    aliases: Vec<String>,

    #[serde(default, rename="localizedAliases")]
    localized_aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEventsContainer {
    #[serde(default)]
    events: Vec<TmEvent>,
}

#[derive(Debug, Serialize, Deserialize, DefaultFromSerde)]
pub struct TmEvents {
    #[serde(default, rename="_links")]
    links: TmPaginatedLinks,

    #[serde(default, rename="_embedded")]
    container: TmEventsContainer,

    #[serde(default)]
    page: TmPagination,
}
