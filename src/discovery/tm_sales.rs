use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSalesPublic {
    #[serde(default, rename="startDateTime")]
    start_date_time: Option<String>,

    #[serde(default, rename="endDateTime")]
    end_date_time: Option<String>,

    #[serde(default, rename="startTBD")]
    start_tbd: bool
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmPresale {
    #[serde(default)]
    name: String,

    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    url: String,

    #[serde(default, rename="startDateTime")]
    start_date_time: String,

    #[serde(default, rename="endDateTime")]
    end_date_time: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmSales {
    #[serde(default)]
    public: TmSalesPublic,
    
    #[serde(default)]
    presales: Vec<TmPresale>
}