use serde::{Deserialize, Serialize};

use getset::{Getters, Setters};

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct EventSearchQuery {
    pub id: Option<String>,
    pub keyword: Option<String>,

    #[serde(rename = "attractionId")]
    pub attraction_id: Option<String>,

    #[serde(rename = "venueId")]
    pub venue_id: Option<String>,

    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,

    pub radius: Option<u32>,

    #[serde(rename = "unit")]
    pub radius_unit: Option<String>,

    pub source: Option<String>,

    pub locale: Option<String>,

    #[serde(rename = "marketId")]
    pub market_id: Option<String>,

    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<String>,

    #[serde(rename = "endDateTime")]
    pub end_date_time: Option<String>,

    #[serde(rename = "includeTBA")]
    pub include_tba: Option<String>,

    #[serde(rename = "includeTBD")]
    pub include_tbd: Option<String>,

    #[serde(rename = "includeText")]
    pub include_test: Option<String>,

    pub size: Option<u32>,

    pub page: Option<u32>,

    pub sort: Option<String>,

    #[serde(rename = "onsaleStartDatetime")]
    pub on_sale_start_date_time: Option<String>,

    #[serde(rename = "onsaleEndDateTime")]
    pub on_sale_end_date_time: Option<String>,

    pub city: Option<Vec<String>>,

    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,

    #[serde(rename = "stateCode")]
    pub state_code: Option<String>,

    #[serde(rename = "classificationName")]
    pub classification_name: Option<Vec<String>>,

    #[serde(rename = "classificationId")]
    pub classification_id: Option<Vec<String>>,

    #[serde(rename = "dmaId")]
    pub dma_id: Option<String>,

    #[serde(rename = "localStartDateTime")]
    pub local_start_date_time: Option<Vec<String>>,

    #[serde(rename = "localStartEndDateTime")]
    pub local_start_end_date_time: Option<Vec<String>>,

    #[serde(rename = "localEndDateTime")]
    pub local_end_date_time: Option<Vec<String>>,

    #[serde(rename = "startEndDateTime")]
    pub start_end_date_time: Option<Vec<String>>,

    #[serde(rename = " publicVisibilityStartDateTime ")]
    pub public_visibility_start_date_time: Option<Vec<String>>,

    #[serde(rename = "preSaleDateTime")]
    pub presale_date_time: Option<Vec<String>>,

    #[serde(rename = "onsaleOnStartDate")]
    pub on_sale_start_date: Option<String>,

    #[serde(rename = "onsaleOnAfterStartDate")]
    pub on_sale_after_start_date: Option<String>,

    #[serde(rename = "collectionId")]
    pub collection_id: Option<Vec<String>>,

    #[serde(rename = "segmentId")]
    pub segment_id: Option<Vec<String>>,

    #[serde(rename = "segmentName")]
    pub segment_name: Option<Vec<String>>,

    #[serde(rename = "includeFamily")]
    pub include_family: Option<String>,

    #[serde(rename = "promoterId")]
    pub promoter_id: Option<String>,

    #[serde(rename = "genreId")]
    pub genre_id: Option<Vec<String>>,

    #[serde(rename = "subGenreId")]
    pub subgenre_id: Option<Vec<String>>,

    #[serde(rename = "typeId")]
    pub type_id: Option<Vec<String>>,

    #[serde(rename = "subTypeId")]
    pub subtype_id: Option<Vec<String>>,

    #[serde(rename = "geoPoint")]
    pub geo_point: Option<String>,

    #[serde(rename = "preferredCountry")]
    pub preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    pub include_spellcheck: Option<String>,

    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct DetailsQuery {
    pub locale: Option<String>,
    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AttractionSearchQuery {
    pub id: Option<String>,
    pub keyword: Option<String>,
    pub source: Option<String>,
    pub locale: Option<String>,

    #[serde(rename = "includeTest")]
    pub include_test: Option<String>,
    pub size: Option<u32>,
    pub page: Option<u32>,
    pub sort: Option<String>,

    #[serde(rename = "classificationName")]
    pub classification_name: Option<Vec<String>>,

    #[serde(rename = "classificationId")]
    pub classification_id: Option<Vec<String>>,

    #[serde(rename = "includeFamily")]
    pub include_family: Option<String>,

    #[serde(rename = "segmentId")]
    pub segment_id: Option<Vec<String>>,

    #[serde(rename = "genreId")]
    pub genre_id: Option<Vec<String>>,

    #[serde(rename = "subGenreId")]
    pub subgenre_id: Option<Vec<String>>,

    #[serde(rename = "typeId")]
    pub type_id: Option<Vec<String>>,

    #[serde(rename = "subTypeId")]
    pub subtype_id: Option<Vec<String>>,

    #[serde(rename = "preferredCountry")]
    pub preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    pub include_spellcheck: Option<String>,

    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ClassificationSearchQuery {
    pub id: Option<String>,
    pub keyword: Option<String>,
    pub source: Option<String>,
    pub locale: Option<String>,

    #[serde(rename = "includeTest")]
    pub include_test: Option<String>,
    pub size: Option<u32>,
    pub page: Option<u32>,
    pub sort: Option<String>,

    #[serde(rename = "preferredCountry")]
    pub preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    pub include_spellcheck: Option<String>,

    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct VenuesSearchQuery {
    pub id: Option<String>,
    pub keyword: Option<String>,
    pub latlong: Option<String>,
    pub radius: Option<String>,
    #[serde(rename = "unit")]
    pub radius_unit: Option<String>,
    pub source: Option<String>,
    pub locale: Option<String>,

    #[serde(rename = "includeTest")]
    pub include_test: Option<String>,
    pub size: Option<u32>,
    pub page: Option<u32>,
    pub sort: Option<String>,

    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,

    #[serde(rename = "stateCode")]
    pub state_code: Option<String>,

    #[serde(rename = "geoPoint")]
    pub geo_point: Option<String>,

    #[serde(rename = "preferredCountry")]
    pub preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    pub include_spellcheck: Option<String>,

    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct FindSuggestQuery {
    pub keyword: Option<String>,
    pub latlong: Option<String>,
    pub radius: Option<String>,
    #[serde(rename = "unit")]
    pub radius_unit: Option<String>,
    pub source: Option<String>,
    pub locale: Option<String>,

    #[serde(rename = "includeTBA")]
    pub include_tba: Option<String>,
    #[serde(rename = "includeTBD")]
    pub include_tbd: Option<String>,

    #[serde(rename = "includeTest")]
    pub include_test: Option<String>,
    pub size: Option<u32>,

    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,

    #[serde(rename = "geoPoint")]
    pub geo_point: Option<String>,

    #[serde(rename = "preferredCountry")]
    pub preferred_country: Option<String>,

    #[serde(rename = "includeSpellcheck")]
    pub include_spellcheck: Option<String>,

    pub domain: Option<Vec<String>>,
}
