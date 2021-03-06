use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct BitField {
    pub name: String,
    pub size: i64,
    pub signed: bool,
}

#[derive(Debug, Clone)]
pub enum SwitchType {
    Packet(String),
    Type(Box<PacketDataType>),
    Unknown(Value),
}

#[derive(Debug, Clone)]
pub enum TypeName {
    Anonymous,
    Named(String),
}

impl From<String> for TypeName {
    fn from(value: String) -> Self {
        Self::Named(value)
    }
}

impl Display for TypeName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeName::Anonymous => f.write_str("Anonymous"),
            TypeName::Named(name) => f.write_str(name.as_str()),
        }
    }
}

impl PartialEq<String> for TypeName {
    fn eq(&self, other: &String) -> bool {
        if let TypeName::Named(name) = self {
            name == other
        } else {
            false
        }
    }
}

/// These data types should be available in every version.
/// However, they won't break anything if not present
/// This can also be known as the Native Types
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum NativeType {
    /// Please read the following link for information on parsing https://wiki.vg/Protocol#VarInt_and_VarLong
    VarInt,
    PString {
        count_type: Box<NativeType>,
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
        end_val: i64,
        metadata_type: Box<PacketDataType>,
    },
    TopBitSetTerminatedArray(Box<PacketDataType>),
    BitField(Vec<BitField>),
    // A set of Name and The Type
    Container(Vec<(TypeName, Box<PacketDataType>)>),
    Switch {
        compare_to: String,
        fields: HashMap<String, SwitchType>,
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

impl Display for NativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = match self {
            NativeType::Bool => "bool",
            NativeType::U8 => "u8",
            NativeType::U16 => "u16",
            NativeType::U32 => "u32",
            NativeType::U64 => "u64",
            NativeType::I8 => "i8",
            NativeType::I16 => "i16",
            NativeType::I32 => "i32",
            NativeType::I64 => "i64",
            NativeType::F32 => "f32",
            NativeType::F64 => "f64",
            NativeType::Uuid => "uuid",
            NativeType::Option(_) => "option",
            NativeType::EntityMetadataLoop { .. } => "entityMetadataLoop",
            NativeType::TopBitSetTerminatedArray(_) => "topbitsetterminatedarray",
            NativeType::BitField(_) => "bitfield",
            NativeType::Container(_) => "container",
            NativeType::Switch { .. } => "switch",
            NativeType::Array { .. } => "array",
            NativeType::Void => "void",
            NativeType::RestBuffer => "restbuffer",
            NativeType::NBT => "nbt",
            NativeType::OptionalNBT => "optionalnbt",
            NativeType::VarInt => "varint",
            NativeType::PString { .. } => "pstring",
            NativeType::Buffer { .. } => "buffer",
        };
        write!(f, "{}", value)
    }
}

impl NativeType {
    pub fn contains_type(name: &str) -> bool {
        matches!(
            name,
            "varint"
                | "pstring"
                | "buffer"
                | "bool"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "f32"
                | "f64"
                | "uuid"
                | "option"
                | "entityMetadataLoop"
                | "topbitsetterminatedarray"
                | "bitfield"
                | "container"
                | "switch"
                | "void"
                | "array"
                | "restbuffer"
                | "nbt"
                | "optionalnbt"
        )
    }
    pub fn new(name: &str, layout: Cow<'_, Value>) -> Option<Self> {
        match name {
            "varint" => Some(NativeType::VarInt),
            "pstring" => Self::new_pstring(layout),
            "buffer" => Self::new_buffer(layout),
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
            "option" => Some(NativeType::Option(build_inner_type(layout.into_owned()))),
            "entityMetadataLoop" => Self::new_entity_metadata_loop(layout),
            "topbitsetterminatedarray" => Self::new_top_bitsetterminated_array(layout),
            "bitfield" => Self::new_bitfield(layout),
            "container" => Self::new_container(layout),

            "switch" => Self::new_switch(layout),
            "void" => Some(NativeType::Void),
            "array" => Self::new_array(layout),

            "restbuffer" => Some(NativeType::RestBuffer),
            "nbt" => Some(NativeType::NBT),
            "optionalnbt" => Some(NativeType::OptionalNBT),
            _ => None,
        }
    }

    pub fn new_pstring(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Object(mut obj) = layout.into_owned() {
            if let Value::String(count_type) = obj.remove("countType").unwrap_or_default() {
                if let Some(count_type) = NativeType::new(&count_type, Cow::Owned(Value::Null)) {
                    return Some(NativeType::PString {
                        count_type: Box::new(count_type),
                    });
                }
            }
        }
        None
    }

    fn new_top_bitsetterminated_array(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Object(mut layout) = layout.into_owned() {
            let inner_type = layout.remove("type").unwrap_or_default();
            let inner_type = build_inner_type(inner_type);
            return Some(NativeType::TopBitSetTerminatedArray(inner_type));
        }
        None
    }

    pub fn new_bitfield(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Array(bit_fields) = layout.into_owned() {
            let bit_fields_vec = bit_fields
                .into_iter()
                .map(|v| serde_json::from_value(v).unwrap())
                .collect();

            Some(NativeType::BitField(bit_fields_vec))
        } else {
            None
        }
    }

    pub fn new_switch(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Object(mut layout) = layout.into_owned() {
            return Some(NativeType::Switch {
                compare_to: layout
                    .remove("compareTo")
                    .unwrap()
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                fields: layout
                    .remove("fields")
                    .and_then(|v| {
                        if let Value::Object(fields) = v {
                            Some(
                                fields
                                    .into_iter()
                                    .map(|(k, v)| {
                                        if let Value::String(value) = v {
                                            if value.starts_with("packet") {
                                                (k, SwitchType::Packet(value))
                                            } else {
                                                (
                                                    k,
                                                    SwitchType::Type(build_inner_type(
                                                        Value::String(value),
                                                    )),
                                                )
                                            }
                                        } else if let Value::Array(array) = v {
                                            (
                                                k,
                                                SwitchType::Type(build_inner_type(Value::Array(
                                                    array,
                                                ))),
                                            )
                                        } else {
                                            (k, SwitchType::Unknown(v))
                                        }
                                    })
                                    .collect(),
                            )
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default(),
                default: layout
                    .remove("default")
                    .map(|v| v.as_str().unwrap_or_default().to_string()),
            });
        }
        None
    }

    pub fn new_array(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Object(mut obj) = layout.into_owned() {
            let value = NativeType::new(
                obj.remove("countType")
                    .unwrap_or_default()
                    .as_str()
                    .unwrap_or_default(),
                Cow::Owned(Value::Null),
            ).unwrap_or(NativeType::VarInt);
            let inner_type = build_inner_type(obj.remove("type").unwrap_or_default());
            return Some(NativeType::Array {
                count_type: Box::new(value),
                array_type: inner_type,
            });
        }
        None
    }

    pub fn new_container(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Array(containers) = layout.into_owned() {
            let containers_vec = containers
                .into_iter()
                .map(|v| {
                    if let Value::Object(mut obj) = v {
                        if let Some(name) = obj.remove("name") {
                            let name = name.as_str().unwrap().to_string();
                            let inner_type = obj.remove("type").unwrap_or_default();
                            let inner_type = build_inner_type(inner_type);
                            (TypeName::Named(name), inner_type)
                        } else {
                            let inner_type = obj.remove("type").unwrap_or_default();
                            let inner_type = build_inner_type(inner_type);
                            (TypeName::Anonymous, inner_type)
                        }
                    } else {
                        panic!("Container is not an object");
                    }
                })
                .collect();

            Some(NativeType::Container(containers_vec))
        } else {
            None
        }
    }

    pub fn new_entity_metadata_loop(layout: Cow<Value>) -> Option<NativeType> {
        match layout.into_owned() {
            Value::Object(mut layout) => {
                let end_val = layout
                    .remove("endVal")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let inner_type = layout.remove("type").unwrap_or_default();
                let inner_type = build_inner_type(inner_type);
                Some(NativeType::EntityMetadataLoop {
                    end_val,
                    metadata_type: inner_type,
                })
            }
            _ => None,
        }
    }

    pub fn new_buffer(layout: Cow<Value>) -> Option<NativeType> {
        if let Value::Object(mut obj) = layout.into_owned() {
            if let Value::String(count_type) = obj.remove("countType").unwrap_or_default() {
                if let Some(count_type) = NativeType::new(&count_type, Cow::Owned(Value::Null)) {
                    return Some(NativeType::PString {
                        count_type: Box::new(count_type),
                    });
                }
            }
        }
        None
    }
}

#[inline]
pub(crate) fn build_inner_type(value: Value) -> Box<PacketDataType> {
    match value {
        Value::String(simple_type) => {
            return if let Some(simple_type) = NativeType::new(&simple_type, Cow::Owned(Value::Null))
            {
                Box::new(PacketDataType::Native(simple_type))
            } else {
                // Probably a reference to a built type
                Box::new(PacketDataType::Other {
                    name: Some(simple_type.into()),
                    value: Value::Null,
                })
            };
        }
        Value::Array(mut array) => {
            if array.len() != 2 {
                return Box::new(PacketDataType::Other {
                    name: None,
                    value: Value::Array(array),
                });
            }
            let inner_value = Cow::Owned(array.pop().unwrap_or_default());
            let key = array.pop().unwrap();
            if let Value::String(key) = key {
                let value = PacketDataType::new(&key, Cow::clone(&inner_value)).or_else(|| {
                    let option = NativeType::new(&key, inner_value.clone());
                    option.map(PacketDataType::Native)
                });
                if let Some(value) = value {
                    Box::new(value)
                } else {
                    Box::new(PacketDataType::Other {
                        name: Some(key.into()),
                        value: inner_value.into_owned(),
                    })
                }
            } else {
                Box::new(PacketDataType::Other {
                    name: None,
                    value: Value::Array(vec![key, inner_value.into_owned()]),
                })
            }
        }
        v => Box::new(PacketDataType::Other {
            name: None,
            value: v,
        }),
    }
}

#[derive(Debug, Clone)]
pub enum PacketDataType {
    // Just a pure native type
    Native(NativeType),
    // It was marked as "native" however, this enum does not have it
    UnknownNativeType(String),
    // This type is built from a native type
    Built {
        // The name of the built type
        name: TypeName,
        // The value of the built type
        value: NativeType,
    },
    /// A Type that could not be built or parsed. This is a fallback for when we don't know what the type is
    /// This type is usually a reference to a built type
    ///
    /// If this is a reference you you will want to push any data within Value to the reference.
    /// For example the Packet type references another type called "PacketData" that has a variable within it for the compareTo value. You will want to take the value from the Value and push it to the PacketData type.
    Other {
        // The name of the type if found
        name: Option<TypeName>,
        // The JSON value of the type
        value: Value,
    },
}

impl From<NativeType> for PacketDataType {
    fn from(native: NativeType) -> Self {
        PacketDataType::Native(native)
    }
}

impl PacketDataType {
    pub fn new(key: &str, value: Cow<'_, Value>) -> Option<Self> {
        if !NativeType::contains_type(key) {
            match value.into_owned() {
                Value::String(string) => Some(PacketDataType::UnknownNativeType(string)),
                Value::Array(mut array) => {
                    if array.len() != 2 {
                        return Some(PacketDataType::Other {
                            name: Some(key.to_string().into()),
                            value: Value::Array(array),
                        });
                    }

                    let inner_type_values = array.pop().unwrap_or_default();
                    let inner_type_name = array.pop().unwrap();
                    if let Value::String(inner_type_name) = inner_type_name {
                        return if let Some(type_) =
                        NativeType::new(&inner_type_name, Cow::Borrowed(&inner_type_values))
                        {
                            Some(PacketDataType::Built {
                                name: TypeName::Named(key.to_string()),
                                value: type_,
                            })
                        } else {
                            Some(PacketDataType::Other {
                                name: Some(inner_type_name.into()),
                                value: inner_type_values,
                            })
                        };
                    }
                    None
                }
                v => Some(PacketDataType::Other {
                    name: Some(key.to_string().into()),
                    value: v,
                }),
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketDataTypes {
    pub types: Vec<PacketDataType>,
}

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use serde::de::MapAccess;

impl<'de> Deserialize<'de> for PacketDataTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct PacketDataTypesVisitor;

        impl<'de> Visitor<'de> for PacketDataTypesVisitor {
            type Value = PacketDataTypes;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct PacketDataTypes")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PacketDataTypes, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut types = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<Value>()?;
                    if let Some(ty) = PacketDataType::new(&key, Cow::Owned(value)) {
                        types.push(ty);
                    }
                }
                Ok(PacketDataTypes { types })
            }
        }

        deserializer.deserialize_map(PacketDataTypesVisitor)
    }
}
