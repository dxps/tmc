use super::{create_id, datatype::DataType};
use chrono::{DateTime, NaiveDate, Utc};

pub trait Attribute {
    fn get_type(&self) -> DataType;
}

#[derive(Debug)]
pub struct AttributeDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_type: DataType,
}

impl AttributeDefinition {
    pub fn new_text(name: String, description: Option<String>) -> Self {
        Self {
            id: create_id(),
            name,
            description,
            data_type: DataType::Text,
        }
    }
}

#[derive(Debug)]
pub struct TextAttribute {
    pub id: String,
    pub name: String,
    pub value: String,
    pub def_id: String,
}

impl TextAttribute {
    pub fn new(name: String, value: String, def_id: String) -> Self {
        Self {
            id: create_id(),
            name,
            value,
            def_id,
        }
    }
}

impl Attribute for TextAttribute {
    fn get_type(&self) -> DataType {
        DataType::Text
    }
}

#[derive(Debug)]
pub struct IntegerAttribute {
    pub name: String,
    pub value: i32,
    pub def_id: String,
}

impl Attribute for IntegerAttribute {
    fn get_type(&self) -> DataType {
        DataType::Integer
    }
}

#[derive(Debug)]
pub struct DateAttribute {
    pub name: String,
    pub value: NaiveDate,
    pub def_id: String,
}

impl Attribute for DateAttribute {
    fn get_type(&self) -> DataType {
        DataType::Date
    }
}

#[derive(Debug)]
pub struct DateTimeAttribute {
    pub name: String,
    pub value: DateTime<Utc>,
    pub def_id: String,
}

impl Attribute for DateTimeAttribute {
    fn get_type(&self) -> DataType {
        DataType::DateTime
    }
}

pub struct EmailAttribute {
    pub name: String,
    pub value: String,
    pub def_id: String,
}

impl Attribute for EmailAttribute {
    fn get_type(&self) -> DataType {
        DataType::Email
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::{Attribute, DataType};

    use super::{AttributeDefinition, TextAttribute};

    #[test]
    fn text_attribute_simple_check() {
        //
        let name = "First name";
        let attr_def = AttributeDefinition::new_text(name.into(), None);
        let attr = TextAttribute::new(attr_def.name, "some text value".into(), attr_def.id);
        assert_eq!(name, attr.name);
        assert_eq!(DataType::Text, attr.get_type())
    }
}
