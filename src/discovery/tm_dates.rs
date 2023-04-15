use either::Either;
use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

use super::tm_structs::default_as_true;
use super::tm_structs::invalid_value_type;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmChronologyZone {
    #[serde(default)]
    pub fixed: bool,

    #[serde(default)]
    pub id: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmChronology {
    #[serde(default)]
    pub zone: TmChronologyZone,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeValue {
    #[serde(default = "invalid_value_type", rename = "type")]
    pub value_type: i32,

    #[serde(default)]
    pub format: i32,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeDurationType {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeFieldType {
    #[serde(default, rename = "durationType")]
    pub duration_type: Option<TmLocalTimeDurationType>,

    #[serde(default, rename = "rangeDurationType")]
    pub range_duration_type: Option<TmLocalTimeDurationType>,

    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeFieldRangeDurationType {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeFieldRangeDuration {
    #[serde(default, rename = "unitMillis")]
    pub unit_millis: i64,

    #[serde(default)]
    pub precise: bool,

    #[serde(default)]
    pub name: String,

    #[serde(default, rename = "type")]
    pub duration_type: TmLocalTimeFieldRangeDurationType,

    #[serde(default = "default_as_true")]
    pub supported: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeField {
    #[serde(default)]
    pub lenient: bool,

    #[serde(default, rename = "rangeDurationField")]
    pub range_duration_field: Option<TmLocalTimeFieldRangeDuration>,

    #[serde(default, rename = "durationField")]
    pub duration_field: Option<TmLocalTimeFieldRangeDuration>,

    #[serde(default, rename = "minimumValue")]
    pub minimum_value: i32,

    #[serde(default, rename = "maximumValue")]
    pub maximum_value: i32,

    #[serde(default, rename = "leapDurationField")]
    pub leap_duration_field: Option<TmLocalTimeFieldRangeDuration>,

    #[serde(default)]
    pub name: String,

    #[serde(default, rename = "type")]
    pub field_type: Option<TmLocalTimeFieldType>,

    #[serde(default = "default_as_true")]
    pub supported: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTime {
    #[serde(default, rename = "millisOfSecond")]
    pub millis_of_second: i32,

    #[serde(default, rename = "millisOfDay")]
    pub millis_of_day: i32,

    #[serde(default, rename = "secondOfMinute")]
    pub second_of_minute: i32,

    #[serde(default, rename = "minuteOfHour")]
    pub minute_of_hour: i32,

    #[serde(default, rename = "hourOfDay")]
    pub hour_of_day: i32,

    #[serde(default)]
    pub chronology: Option<TmChronology>,

    #[serde(default)]
    pub values: Vec<TmLocalTimeValue>,

    #[serde(default, rename = "fieldTypes")]
    pub field_types: Vec<TmLocalTimeFieldType>,

    #[serde(default)]
    pub fields: Vec<TmLocalTimeField>,

    #[serde(default, rename = "dateTime")]
    pub date_time: String,

    #[serde(default, rename = "dateTBD")]
    pub date_tbd: bool,

    #[serde(default, rename = "dateTBA")]
    pub date_tba: bool,

    #[serde(default, rename = "noSpecificTime")]
    pub no_specific_time: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmDate {
    #[serde(default, rename = "localDate")]
    pub local_date: Option<String>,

    #[serde(
        default,
        with = "either::serde_untagged_optional",
        rename = "localTime"
    )]
    pub local_time: Option<Either<String, TmLocalTime>>,

    #[serde(default, rename = "dateTime")]
    pub date_time: Option<String>,

    #[serde(default, rename = "dateTBD")]
    pub date_tbd: bool,

    #[serde(default, rename = "dateTBA")]
    pub date_tba: bool,

    #[serde(default, rename = "timeTBA")]
    pub time_tba: bool,

    #[serde(default, rename = "noSpecificTime")]
    pub no_specific_time: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmEventAccess {
    #[serde(default, rename = "startDateTime")]
    pub start_date_time: String,

    #[serde(default, rename = "startApproximate")]
    pub start_approximate: bool,

    #[serde(default, rename = "endDateTime")]
    pub end_date_time: String,

    #[serde(default, rename = "endApproximate")]
    pub end_approximate: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmEventStatus {
    #[serde(default)]
    pub code: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmDates {
    #[serde(default)]
    pub start: Option<TmDate>,

    #[serde(default)]
    pub end: Option<TmDate>,

    #[serde(default)]
    pub access: Option<TmEventAccess>,

    #[serde(default)]
    pub timezone: Option<String>,

    #[serde(default)]
    pub status: Option<TmEventStatus>,

    #[serde(default, rename = "spanMultipleDays")]
    pub span_multiple_days: bool,
}
