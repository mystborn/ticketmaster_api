use either::Either;
use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;

use super::tm_structs::default_as_true;
use super::tm_structs::invalid_value_type;

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmChronologyZone {
    #[serde(default)]
    fixed: bool,

    #[serde(default)]
    id: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmChronology {
    #[serde(default)]
    zone: TmChronologyZone
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeValue {
    #[serde(default="invalid_value_type", rename="type")]
    value_type: i32,

    #[serde(default)]
    format: i32
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeDurationType {
    #[serde(default)]
    name: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeFieldType {
    #[serde(default, rename="durationType")]
    duration_type: Option<TmLocalTimeDurationType>,

    #[serde(default, rename="rangeDurationType")]
    range_duration_type: Option<TmLocalTimeDurationType>,

    #[serde(default)]
    name: Option<String>
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeFieldRangeDurationType {
    #[serde(default)]
    name: String
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeFieldRangeDuration {
    #[serde(default, rename="unitMillis")]
    unit_millis: i64,

    #[serde(default)]
    precise: bool,

    #[serde(default)]
    name: String,

    #[serde(default, rename="type")]
    duration_type: TmLocalTimeFieldRangeDurationType,

    #[serde(default="default_as_true")]
    supported: bool
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTimeField {
    #[serde(default)]
    lenient: bool,

    #[serde(default, rename="rangeDurationField")]
    range_duration_field: Option<TmLocalTimeFieldRangeDuration>,

    #[serde(default, rename="durationField")]
    duration_field: Option<TmLocalTimeFieldRangeDuration>,

    #[serde(default, rename="minimumValue")]
    minimum_value: i32,

    #[serde(default, rename="maximumValue")]
    maximum_value: i32,

    #[serde(default, rename="leapDurationField")]
    leap_duration_field: Option<TmLocalTimeFieldRangeDuration>,

    #[serde(default)]
    name: String,

    #[serde(default, rename="type")]
    field_type: Option<TmLocalTimeFieldType>,

    #[serde(default="default_as_true")]
    supported: bool
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmLocalTime {
    #[serde(default, rename="millisOfSecond")]
    millis_of_second: i32,

    #[serde(default, rename="millisOfDay")]
    millis_of_day: i32,

    #[serde(default, rename="secondOfMinute")]
    second_of_minute: i32,

    #[serde(default, rename="minuteOfHour")]
    minute_of_hour: i32,

    #[serde(default, rename="hourOfDay")]
    hour_of_day: i32,

    #[serde(default)]
    chronology: Option<TmChronology>,

    #[serde(default)]
    values: Vec<TmLocalTimeValue>,

    #[serde(default, rename="fieldTypes")]
    field_types: Vec<TmLocalTimeFieldType>,

    #[serde(default)]
    fields: Vec<TmLocalTimeField>,

    #[serde(default, rename="dateTime")]
    date_time: String,

    #[serde(default, rename="dateTBD")]
    date_tbd: bool,

    #[serde(default, rename="dateTBA")]
    date_tba: bool,

    #[serde(default, rename="noSpecificTime")]
    no_specific_time: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmDate {
    #[serde(default, rename="localDate")]
    local_date: Option<String>,

    #[serde(default, with="either::serde_untagged_optional", rename="localTime")]
    local_time: Option<Either<String, TmLocalTime>>,

    #[serde(default, rename="dateTime")]
    date_time: Option<String>,

    #[serde(default, rename="dateTBD")]
    date_tbd: bool,

    #[serde(default, rename="dateTBA")]
    date_tba: bool,

    #[serde(default, rename="timeTBA")]
    time_tba: bool,

    #[serde(default, rename="noSpecificTime")]
    no_specific_time: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmEventAccess {
    #[serde(default, rename="startDateTime")]
    start_date_time: String,

    #[serde(default, rename="startApproximate")]
    start_approximate: bool,

    #[serde(default, rename="endDateTime")]
    end_date_time: String,

    #[serde(default, rename="endApproximate")]
    end_approximate: bool,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmEventStatus {
    #[serde(default)]
    code: String,
}

#[derive(Debug, DefaultFromSerde, Serialize, Deserialize)]
pub struct TmDates {
    #[serde(default)]
    start: Option<TmDate>,

    #[serde(default)]
    end: Option<TmDate>,

    #[serde(default)]
    access: Option<TmEventAccess>,

    #[serde(default)]
    timezone: Option<String>,

    #[serde(default)]
    status: Option<TmEventStatus>,

    #[serde(default, rename="spanMultipleDays")]
    span_multiple_days: bool
}