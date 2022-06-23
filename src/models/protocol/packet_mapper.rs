use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::num::ParseIntError;

use serde::de::{SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct PacketMapper {
    /// A Type
    #[serde(rename = "type")]
    pub map_type: String,
    /// The first Value is a Hex value of the packet id. That is a string in the JSON. You can convert the map with the `i32::from_str_radix` (The ids do start with 0x) function. You can also just do try_into::<HashMap<i32, String>() on the PacketMapper
    /// The second Value is the name of the packet
    pub mappings: HashMap<String, String>,
}

impl TryInto<HashMap<i32, String>> for PacketMapper {
    type Error = ParseIntError;

    fn try_into(self) -> Result<HashMap<i32, String>, Self::Error> {
        let mut map = HashMap::with_capacity(self.mappings.len());
        for (key, value) in self.mappings.into_iter() {
            let key = i32::from_str_radix(key.trim_start_matches("0x"), 16)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PacketSwitch {
    #[serde(rename = "compareTo")]
    pub compare_to: String,
    /// First value is the name of the packet. Second is the name of the JSON object for the packet.
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PacketMapperSwitch {
    pub mapper: PacketMapper,
    pub switch: PacketSwitch,
}

impl<'de> Deserialize<'de> for PacketMapperSwitch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PacketMapperSwitchVisitor;

        impl<'de> Visitor<'de> for PacketMapperSwitchVisitor {
            type Value = PacketMapperSwitch;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expected a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut mapper = None;
                let mut switch = None;
                while let Some(value) = seq.next_element::<Value>()? {
                    if let Value::Object(mut value) = value {
                        let value = value
                            .remove("type")
                            .ok_or_else(|| de::Error::missing_field("type"))?;
                        if let Value::Array(mut array) = value {
                            let value = array
                                .pop()
                                .ok_or_else(|| de::Error::missing_field("missing content"))?;
                            let key = array
                                .pop()
                                .ok_or_else(|| de::Error::missing_field("missing key"))?;
                            if let Value::String(key) = key {
                                if key.eq("mapper") {
                                    let value: PacketMapper =
                                        serde_json::from_value(value).map_err(de::Error::custom)?;
                                    mapper = Some(value);
                                } else if key.eq("switch") {
                                    let value: PacketSwitch =
                                        serde_json::from_value(value).map_err(de::Error::custom)?;
                                    switch = Some(value);
                                } else {
                                    return Err(de::Error::custom("unknown key"));
                                }
                            } else {
                                return Err(de::Error::custom("unknown key"));
                            }
                        } else {
                            return Err(de::Error::custom("Expected an array"));
                        }
                    }
                }
                let map_type = mapper.ok_or_else(|| de::Error::missing_field("mapper"))?;
                let switch = switch.ok_or_else(|| de::Error::missing_field("switch"))?;
                Ok(PacketMapperSwitch {
                    mapper: map_type,
                    switch,
                })
            }
        }

        deserializer.deserialize_seq(PacketMapperSwitchVisitor)
    }
}
