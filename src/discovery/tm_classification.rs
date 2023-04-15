use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;

use super::{TmPaginatedLinks, TmPagination, TmLink, TmSimpleLinks};

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
    #[serde(default, rename="_links")]
    links: Option<TmSimpleLinks>,

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
pub struct TmSubgenreDetails {

    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSubgenresDetailsContainer {

    #[serde(default)]
    subgenres: Vec<TmSubgenreDetails>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmGenreDetails {

    #[serde(default, rename="_embedded")]
    subgenres: TmSubgenresDetailsContainer,

    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmGenresDetailsContainer {

    #[serde(default)]
    genres: Vec<TmGenreDetails>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSegmentDetails {

    #[serde(default, rename="_embedded")]
    genres: TmGenresDetailsContainer,

    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSubtypeDetails {

    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(default)]
    locale: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationDetails {

    #[serde(default, rename="_links")]
    links: TmSimpleLinks,

    #[serde(default)]
    segment: TmSegmentDetails,

    #[serde(default)]
    primary: bool,

    #[serde(default)]
    classification_type: TmClassificationType,

    #[serde(default, rename="subType")]
    subtype: TmSubtypeDetails,

    #[serde(default)]
    family: bool
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationsSearchContainer {

    #[serde(default)]
    classications: Vec<TmClassificationDetails>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationsSearch {
    #[serde(default, rename="_links")]
    links: TmPaginatedLinks,

    #[serde(default, rename="_embedded")]
    container: TmClassificationsSearchContainer,

    #[serde(default)]
    page: TmPagination
}