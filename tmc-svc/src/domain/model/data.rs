use serde::Serialize;

use super::create_id;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Text,
    Integer,
    Date,
    DateTime,
    Email,
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "text" => Self::Text,
            "integer" => Self::Integer,
            "date" => Self::Date,
            "datetime" => Self::DateTime,
            "email" => Self::Email,
            _ => DataType::Text,
        }
    }
}

impl Serialize for DataType {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DataType::Text => ser.serialize_str("text"),
            DataType::Integer => ser.serialize_str("integer"),
            DataType::Date => ser.serialize_str("date"),
            DataType::DateTime => ser.serialize_str("datetime"),
            DataType::Email => ser.serialize_str("email"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataValidation {
    TextLengthMin(u8),
    TextLengthMinMax(u8, u8),
}

pub trait DataItem {
    fn get_datatype(&self) -> DataType;
}

/// The definition of a data item.
/// Data item is a basic / foundation element that stores some data.
#[derive(Debug, PartialEq, Serialize)]
pub struct DataItemDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_type: DataType,
}

impl DataItemDef {
    pub fn new_text(name: String, description: Option<String>) -> Self {
        Self {
            id: create_id(),
            name,
            description,
            data_type: DataType::Text,
        }
    }
}
