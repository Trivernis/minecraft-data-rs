use std::collections::HashMap;
use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;
use serde_json::Value;

pub struct BitField {
    name: String,
    size: usize,
    signed: bool,
}

/// These data types should be available in every version.
/// However, they won't break anything if not present
/// This can also be known as the Native Types
pub enum NativeType {
    /// Please read the following link for information on parsing https://wiki.vg/Protocol#VarInt_and_VarLong
    VarInt,
    PString {
        count_type: Box<NativeType>
    },
    Buffer {
        count_type: Box<NativeType>,
    },
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Uuid,
    // Optional<MinecraftPacketDataType>
    Option(Box<PacketDataType>),
    EntityMetadataLoop {
        end_val: i32,
        metadata_type: Box<NativeType>,
    },
    TopBitSetTerminatedArray(Box<NativeType>),
    BitField(Vec<BitField>),
    // A set of Name and The Type
    Container(Vec<(String, Box<PacketDataType>)>),
    Switch {
        compare_to: String,
        fields: HashMap<String, String>,
        default: Option<String>,
    },
    Void,
    Array {
        count_type: Box<NativeType>,
        array_type: Box<PacketDataType>,
    },
    RestBuffer,
    NBT,
    OptionalNBT,
}

impl NativeType {
    pub fn contains_type(name: &str) -> bool {
        match name {
            "varint" => true,
            "pstring" => true,
            "buffer" => true,
            "bool" => true,
            "u8" => true,
            "u16" => true,
            "u32" => true,
            "u64" => true,
            "i8" => true,
            "i16" => true,
            "i32" => true,
            "i64" => true,
            "f32" => true,
            "f64" => true,
            "uuid" => true,
            "option" => true,
            "entitymetadataloop" => true,
            "topbitsetterminatedarray" => true,
            "bitfield" => true,
            "container" => true,
            "switch" => true,
            "void" => true,
            "array" => true,
            "restbuffer" => true,
            "nbt" => true,
            "optionalnbt" => true,
            _ => false,
        }
    }
    pub fn new(name: &str, layout: Option<Value>) -> Option<Self> {
        match name {
            "varint" => Some(NativeType::VarInt),
            "pstring" => {
                if let Some(layout) = layout {
                    if let Some(value) = &layout.as_object().unwrap().get("countType") {
                        if let Some(count_type) = NativeType::new(value.as_str().unwrap(), None) {
                            Some(NativeType::PString {
                                count_type: Box::new(count_type),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "buffer" => {
                if let Some(layout) = layout {
                    if let Some(count_type) = NativeType::new(&layout["countType"].as_str().unwrap(), None) {
                        Some(NativeType::Buffer {
                            count_type: Box::new(count_type),
                        })
                    } else {
                        None
                    }
                } else { None }
            }
            "bool" => Some(NativeType::Bool),
            "u8" => Some(NativeType::U8),
            "u16" => Some(NativeType::U16),
            "u32" => Some(NativeType::U32),
            "u64" => Some(NativeType::U64),
            "i8" => Some(NativeType::I8),
            "i16" => Some(NativeType::I16),
            "i32" => Some(NativeType::I32),
            "i64" => Some(NativeType::I64),
            "f32" => Some(NativeType::F32),
            "f64" => Some(NativeType::F64),
            "uuid" => Some(NativeType::Uuid),
            "option" => {
                if let Some(layout) = layout {
                    let option = layout.as_array().unwrap().get(1);

                    if let Some(option_type) = option {
                        let key = option_type.as_str().unwrap();
                        let value = PacketDataType::new(key, None).or(Self::new(key, None).and_then(|x| Some(PacketDataType::Native(x))));
                        Some(NativeType::Option(Box::new(value.unwrap())))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "entitymetadataloop" => {
                if let Some(layout) = layout {
                    if let Some(end_val) = layout["endVal"].as_i64() {
                        let value1 = layout["type"].as_array().unwrap();
                        if let Some(metadata_type) = NativeType::new(&value1.get(0).unwrap().to_string(), value1.get(1).cloned()) {
                            Some(NativeType::EntityMetadataLoop {
                                end_val: end_val as i32,
                                metadata_type: Box::new(metadata_type),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "topbitsetterminatedarray" => {
                if let Some(layout) = layout {
                    if let Some(count_type) = NativeType::new(&layout["countType"].as_str().unwrap(), None) {
                        Some(NativeType::TopBitSetTerminatedArray(Box::new(count_type)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            "bitfield" => {
                if let Some(layout) = layout {
                    let bit_fields = layout.as_array().unwrap();
                    let mut bit_fields_vec = Vec::new();
                    for bit_field in bit_fields {
                        if let Some(name) = bit_field["name"].as_str() {
                            if let Some(size) = bit_field["size"].as_i64() {
                                if let Some(signed) = bit_field["signed"].as_bool() {
                                    bit_fields_vec.push(BitField {
                                        name: name.to_string(),
                                        size: size as usize,
                                        signed: signed,
                                    });
                                }
                            }
                        }
                    }
                    Some(NativeType::BitField(bit_fields_vec))
                } else {
                    None
                }
            }
            "container" => {
                if let Some(layout) = layout {
                    let containers = layout.as_array().unwrap();
                    let mut containers_vec = Vec::new();
                    for container in containers {
                        if let Some(name) = container["name"].as_str() {
                            if let Some(type_) = container["type"].as_str() {
                                containers_vec.push((name.to_string(), Box::new(PacketDataType::new(type_, None).or(Self::new(type_, None).
                                    and_then(|x| Some(PacketDataType::Native(x)))).unwrap())));
                            }
                        }
                    }
                    Some(NativeType::Container(containers_vec))
                } else {
                    None
                }
            }
            "switch" => {
                if let Some(layout) = layout {
                    if let Some(name) = layout["compareTo"].as_str() {
                        if let Some(fields) = layout["fields"].as_object() {
                            let fields = fields.iter().map(|(key, v)| {
                                (key.to_string(), v.to_string())
                            }).collect();
                            return Some(NativeType::Switch {
                                compare_to: name.to_string(),
                                fields: fields,
                                default: None,
                            });
                        }
                    }
                }
                None
            }
            "void" => Some(NativeType::Void),
            "array" => {
                if let Some(layout) = layout {
                    let value = layout.as_object().unwrap();
                    if let Some(count_type) = NativeType::new(&value.get("countType").unwrap().as_str().unwrap(), None) {
                        let type_ = value.get("type").unwrap();
                         if let Some(type_) = &type_.as_str() {
                           return Some(NativeType::Array {
                                count_type: Box::new(count_type),
                                array_type: Box::new(PacketDataType::new(type_, None).or(Self::new(type_, None).
                                    and_then(|x| Some(PacketDataType::Native(x)))).unwrap()),
                            });
                        } else if let Some(array) = type_.as_array() {
                             let key = array.get(0).unwrap().as_str().unwrap();
                             if let Some(inner_type) = PacketDataType::new(key, array.get(1).cloned()).
                                 or(Self::new(key, array.get(1).cloned()).and_then(|x| Some(PacketDataType::Native(x)))) {
                               return Some(NativeType::Array {
                                    count_type: Box::new(count_type),
                                    array_type: Box::new(inner_type),
                                });
                            }else{
                                 println!("Could not parse array type: {}", key);
                             }

                        }
                    }else{
                        return None;
                    }
                }
                return None;
            }

            "restbuffer" => Some(NativeType::RestBuffer),
            "nbt" => Some(NativeType::NBT),
            "optionalnbt" => Some(NativeType::OptionalNBT),
            _ => None,
        }
    }
}

pub enum PacketDataType {
    // Just a pure native type
    Native(NativeType),
    // It was marked as "native" however, this enum does not have it
    UnknownNativeType(String),
    // This type is built from a native type
    Built(NativeType),

    Other(String, Value),
}

impl PacketDataType {
    pub fn new(key: &str, value: Option<Value>) -> Option<Self> {
        if !NativeType::contains_type(&key) {
            let value = value.unwrap_or_default();
            if value.is_string() {
                Some(PacketDataType::UnknownNativeType(key.to_string()))
            } else if let Some(array) = value.as_array() {
                if let Some(name) = array.get(0) {
                    if let Some(name) = name.as_str() {
                        let option = value.get(1).cloned();
                        let other_type = NativeType::new(&name, option.clone());
                        if let Some(type_) = other_type {
                            Some(PacketDataType::Built(type_))
                        } else {
                            Some(PacketDataType::Other(name.to_string(), option.unwrap_or_default()))
                        }
                    } else {
                        Some(PacketDataType::Other(key.to_string(), value))
                    }
                } else {
                    None
                }
            } else {
                Some(PacketDataType::Other(key.to_string(), value))
            }
        } else {
            None
        }
    }
}

pub struct PacketDataTypes {
    pub types: Vec<PacketDataType>,
}

use std::fmt;

use serde::de::{self, SeqAccess, MapAccess};

impl<'de> Deserialize<'de> for PacketDataTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct PacketDataTypesVisitor;

        impl<'de> Visitor<'de> for PacketDataTypesVisitor {
            type Value = PacketDataTypes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PacketDataTypes")
            }


            fn visit_map<V>(self, mut map: V) -> Result<PacketDataTypes, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut types = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<Value>()?;
                    if let Some(ty) = PacketDataType::new(&key, Some(value)) {
                        types.push(ty);
                    }
                }
                Ok(PacketDataTypes { types })
            }
        }

        deserializer.deserialize_map(PacketDataTypesVisitor)
    }
}