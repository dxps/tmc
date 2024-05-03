use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::model::{create_id, DataType};

use super::DataItem;

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

impl DataItem for TextAttribute {
    fn get_datatype(&self) -> DataType {
        DataType::Text
    }
}

#[derive(Debug)]
pub struct IntegerAttribute {
    pub name: String,
    pub value: i32,
    pub def_id: String,
}

impl DataItem for IntegerAttribute {
    fn get_datatype(&self) -> DataType {
        DataType::Integer
    }
}

#[derive(Debug)]
pub struct DateAttribute {
    pub name: String,
    pub value: NaiveDate,
    pub def_id: String,
}

impl DataItem for DateAttribute {
    fn get_datatype(&self) -> DataType {
        DataType::Date
    }
}

#[derive(Debug)]
pub struct DateTimeAttribute {
    pub name: String,
    pub value: DateTime<Utc>,
    pub def_id: String,
}

impl DataItem for DateTimeAttribute {
    fn get_datatype(&self) -> DataType {
        DataType::DateTime
    }
}

pub struct EmailAttribute {
    pub name: String,
    pub value: String,
    pub def_id: String,
}

impl DataItem for EmailAttribute {
    fn get_datatype(&self) -> DataType {
        DataType::Email
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::{DataItem, DataItemDef, DataType};

    use super::TextAttribute;

    #[test]
    fn text_attribute_simple_check() {
        //
        let name = "First name";
        let attr_def = DataItemDef::new_text(name.into(), None);
        let attr = TextAttribute::new(attr_def.name, "some text value".into(), attr_def.id);
        assert_eq!(name, attr.name);
        assert_eq!(DataType::Text, attr.get_datatype())
    }
}
