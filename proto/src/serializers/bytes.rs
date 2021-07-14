//! Serialize/deserialize bytes (Vec<u8>) type

extern crate prusti_contracts;


/// Serialize into hexstring, deserialize from hexstring
pub mod hexstring {
    use prusti_contracts::*;
    use serde::{Deserialize, Deserializer, Serializer};
    use subtle_encoding::hex;

    /// Deserialize hexstring into Vec<u8>
    #[trusted]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        let upper = hex::decode_upper(&string);
        if upper.is_ok() {
            upper.map_err(serde::de::Error::custom)
        } else {
            hex::decode(&string).map_err(serde::de::Error::custom)
        }
    }

    /// Serialize from T into hexstring
    #[trusted]
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let hex_bytes = hex::encode_upper(value.as_ref());
        let hex_string = String::from_utf8(hex_bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&hex_string)
    }
}

/// Serialize into base64string, deserialize from base64string
pub mod base64string {
    use prusti_contracts::*;
    use serde::{Deserialize, Deserializer, Serializer};
    use subtle_encoding::base64;

    /// Deserialize base64string into Vec<u8>
    #[trusted]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        base64::decode(&string).map_err(serde::de::Error::custom)
    }

    /// Deserialize base64string into String
    #[trusted]
    pub fn deserialize_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        String::from_utf8(base64::decode(&s).map_err(serde::de::Error::custom)?)
            .map_err(serde::de::Error::custom)
    }

    /// Serialize from T into base64string
    #[trusted]
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let base64_bytes = base64::encode(value.as_ref());
        let base64_string = String::from_utf8(base64_bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&base64_string)
    }
}

/// Serialize into Vec<base64string>, deserialize from Vec<base64string>
pub mod vec_base64string {
    use prusti_contracts::*;
    use serde::{Deserializer, Serializer};
    use subtle_encoding::base64;

    /// Deserialize array into Vec<Vec<u8>>
    #[trusted]
    pub fn deserialize<'de, D>(_deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::new())
        // Option::<Vec<String>>::deserialize(deserializer)?
        //     .unwrap_or_default()
        //     .into_iter()
        //     .map(|s| base64::decode(&s).map_err(serde::de::Error::custom))
        //     .collect()
    }

    /// Serialize from Vec<T> into Vec<base64string>
    #[trusted]
    pub fn serialize<S, T>(value: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let base64_strings = value
            .iter()
            .map(|v| {
                String::from_utf8(base64::encode(v.as_ref())).map_err(|e| serde::ser::Error::custom(e))
            })
            .collect::<Result<Vec<String>, S::Error>>()?;
        serializer.collect_seq(base64_strings)
    }
}

/// Serialize into Option<base64string>, deserialize from Option<base64string>
pub mod option_base64string {
    use prusti_contracts::*;
    use serde::{Deserialize, Deserializer, Serializer};
    use subtle_encoding::base64;

    /// Deserialize Option<base64string> into Vec<u8> or null
    #[trusted]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        base64::decode(&string).map_err(serde::de::Error::custom)
    }

    /// Serialize from T into Option<base64string>
    #[trusted]
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let base64_bytes = base64::encode(value.as_ref());
        let base64_string = String::from_utf8(base64_bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&base64_string)
    }
}

/// Serialize into string, deserialize from string
pub mod string {
    use prusti_contracts::*;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Deserialize string into Vec<u8>
    #[allow(dead_code)]
    #[trusted]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        Ok(string.as_bytes().to_vec())
    }

    /// Serialize from T into string
    #[allow(dead_code)]
    #[trusted]
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let string =
            String::from_utf8(value.as_ref().to_vec()).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&string)
    }
}
