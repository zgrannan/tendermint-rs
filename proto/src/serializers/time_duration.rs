//! Serialize/deserialize std::time::Duration type from and into string:
extern crate prusti_contracts;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use std::time::Duration;
use prusti_contracts::*;

/// Deserialize string into Duration
#[trusted]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|e| D::Error::custom(e))?;

    Ok(Duration::from_nanos(value))
}

/// Serialize from Duration into string
#[trusted]
pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    format!("{}", value.as_nanos()).serialize(serializer)
}
