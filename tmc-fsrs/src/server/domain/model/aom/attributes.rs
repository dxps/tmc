use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    pub id: String,
    /// The definition id.
    pub did: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttributeCategory {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttributeDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub data_type: AttributeDataType,
    pub default_value: String,
    pub is_required: bool,
    pub category_id: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeDataType {
    Text,
    Textarea,
    Number,
    Boolean,
    Date,
    DateTime,
    Time,
    List,
}
