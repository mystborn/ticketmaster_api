use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;

use super::{TmPaginatedLinks, TmPagination};

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmGenre {
    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSegmentGenre {
    #[serde(default, rename="subGenres")]
    sub_genres: Vec<TmGenre>,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSegment {

    #[serde(default)]
    genres: Vec<TmSegmentGenre>,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationSubtype {
    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationType {
    #[serde(default, rename="subTypes")]
    subtypes: Vec<TmClassificationSubtype>,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassification {
    #[serde(default)]
    primary: bool,

    #[serde(default)]
    segment: Option<TmSegment>,

    #[serde(default)]
    genre: Option<TmGenre>,

    #[serde(default, rename="subGenre")]
    sub_genre: Option<TmGenre>,

    #[serde(default, rename="type")]
    classification_type: Option<TmClassificationType>,

    #[serde(default, rename="subType")]
    classification_subtype: Option<TmClassificationSubtype>,

    #[serde(default)]
    family: bool
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationsContainer {
    #[serde(default)]
    classifications: Vec<TmClassification>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassifications {
    #[serde(default, rename="_links")]
    links: TmPaginatedLinks,

    #[serde(default, rename="_embedded")]
    container: TmClassificationsContainer,

    #[serde(default)]
    page: TmPagination
}