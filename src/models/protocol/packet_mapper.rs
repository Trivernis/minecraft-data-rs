use std::collections::HashMap;
use std::fmt;

use serde::de::{SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct PacketMapper {
    /// A Type
    #[serde(rename = "type")]
    pub map_type: String,
    pub mappings: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct PacketSwitch {
    #[serde(rename = "compareTo")]
    pub compare_to: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug)]
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
                        let value = value.remove("type").ok_or_else(|| de::Error::missing_field("type"))?;
                        if let Value::Array(mut array) = value {
                            let value = array.pop().ok_or_else(|| de::Error::missing_field("missing content"))?;
                            let key = array.pop().ok_or_else(|| de::Error::missing_field("missing key"))?;
                            if let Value::String(key) = key {
                                if key.eq("mapper") {
                                    let value: PacketMapper = serde_json::from_value(value).map_err(de::Error::custom)?;
                                    mapper = Some(value);
                                } else if key.eq("switch") {
                                    let value: PacketSwitch = serde_json::from_value(value).map_err(de::Error::custom)?;
                                    switch = Some(value);
                                }else{
                                    return Err(de::Error::custom("unknown key"));
                                }
                            }else{
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
