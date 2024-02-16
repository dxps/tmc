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

#[derive(Debug, PartialEq)]
pub enum DataValidation {
    TextLengthMin(u8),
    TextLengthMinMax(u8, u8),
}
