use serde::{Serialize, Deserialize};

use getset::{Getters, Setters};

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct EventSearchQuery {
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
    dma_id: Option<String>,

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

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct DetailsQuery {
    locale: Option<String>,
    domain: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AttractionSearchQuery {
    id: Option<String>,
    keyword: Option<String>,
    source: Option<String>,
    locale: Option<String>,

    #[serde(rename = "includeTest")]
    include_test: Option<String>,
    size: Option<u32>,
    page: Option<u32>,
    sort: Option<String>,

    #[serde(rename = "classificationName")]
    classification_name: Option<Vec<String>>,

    #[serde(rename = "classificationId")]
    classification_id: Option<Vec<String>>,

    #[serde(rename = "includeFamily")]
    include_family: Option<String>,

    #[serde(rename = "segmentId")]
    segment_id: Option<Vec<String>>,

    #[serde(rename = "genreId")]
    genre_id: Option<Vec<String>>,

    #[serde(rename = "subGenreId")]
    subgenre_id: Option<Vec<String>>,

    #[serde(rename = "typeId")]
    type_id: Option<Vec<String>>,

    #[serde(rename = "subTypeId")]
    subtype_id: Option<Vec<String>>,

    #[serde(rename = "preferredCountry")]
    preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    include_spellcheck: Option<String>,

    domain: Option<Vec<String>>

}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ClassificationSearchQuery {
    id: Option<String>,
    keyword: Option<String>,
    source: Option<String>,
    locale: Option<String>,

    #[serde(rename = "includeTest")]
    include_test: Option<String>,
    size: Option<u32>,
    page: Option<u32>,
    sort: Option<String>,

    #[serde(rename = "preferredCountry")]
    preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    include_spellcheck: Option<String>,

    domain: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct VenuesSearchQuery {
    id: Option<String>,
    keyword: Option<String>,
    latlong: Option<String>,
    radius: Option<String>,
    #[serde(rename="unit")]
    radius_unit: Option<String>,
    source: Option<String>,
    locale: Option<String>,

    #[serde(rename = "includeTest")]
    include_test: Option<String>,
    size: Option<u32>,
    page: Option<u32>,
    sort: Option<String>,

    #[serde(rename = "countryCode")]
    country_code: Option<String>,

    #[serde(rename="stateCode")]
    state_code: Option<String>,

    #[serde(rename = "geoPoint")]
    geo_point: Option<String>,

    #[serde(rename = "preferredCountry")]
    preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    include_spellcheck: Option<String>,

    domain: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct FindSuggestQuery {
    keyword: Option<String>,
    latlong: Option<String>,
    radius: Option<String>,
    #[serde(rename="unit")]
    radius_unit: Option<String>,
    source: Option<String>,
    locale: Option<String>,

    #[serde(rename="includeTBA")]
    include_tba: Option<String>,
    #[serde(rename="includeTBD")]
    include_tbd: Option<String>,

    #[serde(rename = "includeTest")]
    include_test: Option<String>,
    size: Option<u32>,

    #[serde(rename = "countryCode")]
    country_code: Option<String>,

    #[serde(rename = "geoPoint")]
    geo_point: Option<String>,

    #[serde(rename = "preferredCountry")]
    preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    include_spellcheck: Option<String>,

    domain: Option<Vec<String>>
}