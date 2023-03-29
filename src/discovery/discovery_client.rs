use serde::{Serialize, Deserialize};

pub struct TicketmasterDiscoveryClient {
    api_key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    id: Option<String>,
    keyword: Option<String>,

    #[serde(rename = "attractionId")]
    attraction_id: Option<String>,

    #[serde(rename = "venueId")]
    venue_id: Option<String>,

    #[serde(rename = "postalCode")]
    postal_code: Option<String>,

    radius: Option<u32>,

    #[serde(rename = "unit")]
    radius_unit: Option<String>,

    source: Option<String>,

    locale: Option<String>,

    #[serde(rename = "marketId")]
    market_id: Option<String>,

    #[serde(rename = "startDateTime")]
    start_date_time: Option<String>,

    #[serde(rename = "endDateTime")]
    end_date_time: Option<String>,

    #[serde(rename = "includeTBA")]
    include_tba: Option<String>,

    #[serde(rename = "includeTBD")]
    include_tbd: Option<String>,

    #[serde(rename = "includeText")]
    include_test: Option<String>,

    size: Option<u32>,

    page: Option<u32>,

    sort: Option<String>,

    #[serde(rename = "onsaleStartDatetime")]
    on_sale_start_date_time: Option<String>,

    #[serde(rename = "onsaleEndDateTime")]
    on_sale_end_date_time: Option<String>,

    city: Option<Vec<String>>,

    #[serde(rename = "countryCode")]
    country_code: Option<String>,

    #[serde(rename = "stateCode")]
    state_code: Option<String>,

    #[serde(rename = "classificationName")]
    classification_name: Option<Vec<String>>,

    #[serde(rename = "classificationId")]
    classification_id: Option<Vec<String>>,

    #[serde(rename = "dmaId")]
    dma_id: String,

    #[serde(rename = "localStartDateTime")]
    local_start_date_time: Option<Vec<String>>,

    #[serde(rename = "localStartEndDateTime")]
    local_start_end_date_time: Option<Vec<String>>,

    #[serde(rename = "localEndDateTime")]
    local_end_date_time: Option<Vec<String>>,
    
    #[serde(rename = "startEndDateTime")]
    start_end_date_time: Option<Vec<String>>,

    #[serde(rename = " publicVisibilityStartDateTime ")]
    public_visibility_start_date_time: Option<Vec<String>>,
    
    #[serde(rename = "preSaleDateTime")]
    presale_date_time: Option<Vec<String>>,

    #[serde(rename = "onsaleOnStartDate")]
    on_sale_start_date: Option<String>,

    #[serde(rename = "onsaleOnAfterStartDate")]
    on_sale_after_start_date: Option<String>,

    #[serde(rename = "collectionId")]
    collection_id: Option<Vec<String>>,

    #[serde(rename = "segmentId")]
    segment_id: Option<Vec<String>>,

    #[serde(rename = "segmentName")]
    segment_name: Option<Vec<String>>,

    #[serde(rename = "includeFamily")]
    include_family: Option<String>,

    #[serde(rename = "promoterId")]
    promoter_id: Option<String>,

    #[serde(rename = "genreId")]
    genre_id: Option<Vec<String>>,

    #[serde(rename = "subGenreId")]
    subgenre_id: Option<Vec<String>>,

    #[serde(rename = "typeId")]
    type_id: Option<Vec<String>>,

    #[serde(rename = "subTypeId")]
    subtype_id: Option<Vec<String>>,

    #[serde(rename = "geoPoint")]
    geo_point: Option<String>,

    #[serde(rename = "preferredCountry")]
    preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    include_spellcheck: Option<String>,

    domain: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLink {
    href: String,
    templated: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLinks {
    #[serde(rename = "self")]
    current_page: ResponseLink,
    next: Option<ResponseLink>,
    prev: Option<ResponseLink>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePage {
    size: u32,

    #[serde(rename = "totalElements")]
    total_elements: u32,

    #[serde(rename = "totalPages")]
    total_pages: u32,
    number: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventVenues {
    venues: Vec<ResponseLink>,
    attractions: Vec<ResponseLink>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEventLinks {
    #[serde(rename = "self")]
    current_page: ResponseLink,

    #[serde(flatten)]
    venues: EventVenues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventVenuesContainer {
    #[serde(flatten)]
    venues: EventVenues
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLocation {
    latitude: f64,
    longitude: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseImage {
    url: String,
    ratio: String,
    width: u32,
    height: u32,
    fallbacl: bool,
    attribution: Option<String>
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEvent {
    #[serde(rename = "_links")]
    links: ResponseEventLinks,

    #[serde(rename = "_embedded")]
    venues: Option<EventVenuesContainer>,

    #[serde(rename = "type")]
    event_type: String,

    distance: Option<f64>,

    #[serde(rename = "units")]
    distance_units: Option<String>,

    location: Option<ResponseLocation>,

    #[serde(rename = "id*")]
    id: String,

    locale: Option<String>,

    name: String,

    description: Option<String>,

    #[serde(rename = "additionalInfo")]
    additional_info: Option<String>,

    url: String,

    images: Vec<ResponseImage>,

    dates
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEvents {
    events: Vec<ResponseEvent>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "_links")]
    links: ResponseLinks,

    page: ResponsePage,

    #[serde(rename = "_embedded")]
    container: ResponseEvents
}

impl TicketmasterDiscoveryClient {
    pub fn new(api_key: String) -> TicketmasterDiscoveryClient {
        TicketmasterDiscoveryClient {
            api_key
        }
    }
}