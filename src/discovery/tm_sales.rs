use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSalesPublic {
    #[serde(default, rename = "startDateTime")]
    pub start_date_time: Option<String>,

    #[serde(default, rename = "endDateTime")]
    pub end_date_time: Option<String>,

    #[serde(default, rename = "startTBD")]
    pub start_tbd: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPresale {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub url: String,

    #[serde(default, rename = "startDateTime")]
    pub start_date_time: String,

    #[serde(default, rename = "endDateTime")]
    pub end_date_time: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSales {
    #[serde(default)]
    pub public: TmSalesPublic,

    #[serde(default)]
    pub presales: Vec<TmPresale>,
}
