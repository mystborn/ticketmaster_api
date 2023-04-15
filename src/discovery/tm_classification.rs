use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

use super::{TmPaginatedLinks, TmPagination, TmSimpleLinks};

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmGenre {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSegmentGenre {
    #[serde(default, rename = "subGenres")]
    pub sub_genres: Vec<TmGenre>,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSegment {
    #[serde(default)]
    pub genres: Vec<TmSegmentGenre>,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationSubtype {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationType {
    #[serde(default, rename = "subTypes")]
    pub subtypes: Vec<TmClassificationSubtype>,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassification {
    #[serde(default, rename = "_links")]
    pub links: Option<TmSimpleLinks>,

    #[serde(default)]
    pub primary: bool,

    #[serde(default)]
    pub segment: Option<TmSegment>,

    #[serde(default)]
    pub genre: Option<TmGenre>,

    #[serde(default, rename = "subGenre")]
    pub sub_genre: Option<TmGenre>,

    #[serde(default, rename = "type")]
    pub classification_type: Option<TmClassificationType>,

    #[serde(default, rename = "subType")]
    pub classification_subtype: Option<TmClassificationSubtype>,

    #[serde(default)]
    pub family: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSubgenreDetails {
    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSubgenresDetailsContainer {
    #[serde(default)]
    pub subgenres: Vec<TmSubgenreDetails>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmGenreDetails {
    #[serde(default, rename = "_embedded")]
    pub subgenres: TmSubgenresDetailsContainer,

    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmGenresDetailsContainer {
    #[serde(default)]
    pub genres: Vec<TmGenreDetails>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSegmentDetails {
    #[serde(default, rename = "_embedded")]
    pub genres: TmGenresDetailsContainer,

    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSubtypeDetails {
    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub locale: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationDetails {
    #[serde(default, rename = "_links")]
    pub links: TmSimpleLinks,

    #[serde(default)]
    pub segment: TmSegmentDetails,

    #[serde(default)]
    pub primary: bool,

    #[serde(default)]
    pub classification_type: TmClassificationType,

    #[serde(default, rename = "subType")]
    pub subtype: TmSubtypeDetails,

    #[serde(default)]
    pub family: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationsSearchContainer {
    #[serde(default)]
    pub classications: Vec<TmClassificationDetails>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmClassificationsSearch {
    #[serde(default, rename = "_links")]
    pub links: TmPaginatedLinks,

    #[serde(default, rename = "_embedded")]
    pub container: TmClassificationsSearchContainer,

    #[serde(default)]
    pub page: TmPagination,
}
