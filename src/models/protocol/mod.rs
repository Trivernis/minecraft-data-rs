pub mod types;
pub mod packet_mapper;

use std::fmt;
use serde::{de, Deserialize, Deserializer};
use serde::de::{MapAccess, Visitor};
use serde_json::Value;
pub use packet_mapper::{PacketMapper, PacketSwitch, PacketMapperSwitch};
pub use types::{BitField, NativeType, PacketDataType, PacketDataTypes};

#[derive(Deserialize, Debug)]
pub struct Protocol {
    pub types: PacketDataTypes,
    pub handshaking: PacketGrouping,
    pub status: PacketGrouping,
    pub login: PacketGrouping,
    pub play: PacketGrouping,
}

#[derive(Deserialize, Debug)]
pub struct PacketGrouping {
    #[serde(rename = "toServer")]
    pub to_server: PacketTypes,
    #[serde(rename = "toClient")]
    pub to_client: PacketTypes,
}

#[derive(Debug)]
pub enum DataTypeReference {
    Simple(String),
    Complex {
        name: String,
        properties: Value,
    },
}

#[derive(Debug)]
pub struct Packet {
    pub name: String,
    pub data: Vec<(String, DataTypeReference)>,
}

#[derive(Debug)]
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
                                        let value = array.pop().ok_or_else(|| de::Error::missing_field("missing content"))?;
                                        let value: PacketMapperSwitch = serde_json::from_value(value).map_err(de::Error::custom)?;
                                        packet_mapper = Some(value);
                                    } else {
                                        return Err(de::Error::custom("Invalid Packet Mapper"));
                                    }
                                } else if let Value::Array(mut array) = value {
                                    let last = array.pop().ok_or_else(|| de::Error::missing_field("missing content"))?;
                                    if let Value::Array(array) = last {
                                        let mut packet_content = Vec::new();
                                        for value in array.into_iter() {
                                            if let Value::Object(mut obj) = value {
                                                let name = obj.remove("name");
                                                let name = if let Some(name) = name {
                                                    name.to_string()
                                                }else{
                                                    "anon".to_string()
                                                };
                                                let value = obj.remove("type").ok_or_else(|| de::Error::custom(format!("Packet ID {} missing type", key)))?;
                                                let value = match value {
                                                    Value::String(simple) => {
                                                        DataTypeReference::Simple(simple)
                                                    }
                                                    Value::Array(mut array) => {
                                                        let properties = array.pop().ok_or_else(|| de::Error::custom(format!("Packet ID {} missing properties", key)))?;
                                                        let name = array.pop().ok_or_else(|| de::Error::custom(format!("Packet ID {} missing name", key)))?.to_string();
                                                        DataTypeReference::Complex {
                                                            name,
                                                            properties,
                                                        }
                                                    }
                                                    _ => return Err(de::Error::custom(format!("Invalid Packet Invalid Type {}", key)))
                                                };
                                                packet_content.push((name, value));
                                            } else {
                                                return Err(de::Error::custom(format!("Invalid Packet Expected Object {}", key)));
                                            }
                                        }
                                        packets.push(Packet {
                                            name: key,
                                            data: packet_content,
                                        });
                                    } else {
                                        return Err(de::Error::custom(format!("Invalid Packet {}", key)));
                                    }
                                } else {
                                    return Err(de::Error::custom(format!("Invalid Packet  Expected Array {}", key)));
                                }
                            }
                        }

                        let packet_mapper = packet_mapper.ok_or_else(|| de::Error::missing_field("packet_mapper"))?;
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
