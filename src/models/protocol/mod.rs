pub mod packet_mapper;
pub mod types;

pub use packet_mapper::{PacketMapper, PacketMapperSwitch, PacketSwitch};
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt;
pub use types::{BitField, NativeType, PacketDataType, PacketDataTypes};

#[derive(Deserialize, Debug, Clone)]
pub struct Protocol {
    pub types: PacketDataTypes,
    pub handshaking: PacketGrouping,
    pub status: PacketGrouping,
    pub login: PacketGrouping,
    pub play: PacketGrouping,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PacketGrouping {
    #[serde(rename = "toServer")]
    pub to_server: PacketTypes,
    #[serde(rename = "toClient")]
    pub to_client: PacketTypes,
}

#[derive(Debug, Clone)]
pub enum DataTypeReference {
    Simple(String),
    Complex { name: String, properties: Value },
}

impl Into<PacketDataType> for DataTypeReference {
    fn into(self) -> PacketDataType {
        let (name, properties) = match self {
            DataTypeReference::Simple(simple) => (simple, Value::Null),
            DataTypeReference::Complex { name, properties } => (name, properties),
        };

        PacketDataType::new(name.as_str(), Cow::Borrowed(&properties))
            .or_else(|| {
                let option = NativeType::new(name.as_str(), Cow::Borrowed(&properties));
                option.map(PacketDataType::Native)
            })
            .unwrap_or_else(|| PacketDataType::Other {
                name: Some(name.into()),
                value: properties,
            })
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub name: String,
    pub data: PacketDataType,
}

#[derive(Debug, Clone)]
pub struct PacketTypes {
    pub packet_mapper: PacketMapperSwitch,
    pub types: Vec<Packet>,
}

impl<'de> Deserialize<'de> for PacketTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PacketTypesVisitor;

        impl<'de> Visitor<'de> for PacketTypesVisitor {
            type Value = PacketTypes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expected a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<String>()? {
                    if key.eq("types") {
                        let mut packets = Vec::new();
                        let mut packet_mapper = None;
                        let value = map.next_value::<Value>()?;

                        if let Value::Object(obj) = value {
                            for (key, value) in obj.into_iter() {
                                if key.eq("packet") {
                                    if let Value::Array(mut array) = value {
                                        let value = array.pop().ok_or_else(|| {
                                            de::Error::missing_field("missing content")
                                        })?;
                                        let value: PacketMapperSwitch =
                                            serde_json::from_value(value)
                                                .map_err(de::Error::custom)?;
                                        packet_mapper = Some(value);
                                    } else {
                                        return Err(de::Error::custom("Invalid Packet Mapper"));
                                    }
                                } else if let Value::Array(array) = value {
                                    let value1 = Value::Array(vec![
                                        Value::String(key.clone()),
                                        Value::Array(array),
                                    ]);
                                    let inner_type = types::build_inner_type(value1);
                                    packets.push(Packet {
                                        name: key,
                                        data: *inner_type,
                                    });
                                } else {
                                    return Err(de::Error::custom(format!(
                                        "Invalid Packet  Expected Array {}",
                                        key
                                    )));
                                }
                            }
                        }

                        let packet_mapper = packet_mapper
                            .ok_or_else(|| de::Error::missing_field("packet_mapper"))?;
                        return Ok(PacketTypes {
                            packet_mapper,
                            types: packets,
                        });
                    }
                }
                Err(de::Error::custom("Expected a types"))
            }
        }

        deserializer.deserialize_map(PacketTypesVisitor)
    }
}
