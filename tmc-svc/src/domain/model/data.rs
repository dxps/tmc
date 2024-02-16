#[derive(Debug, PartialEq)]
pub enum DataType {
    Text,
    Integer,
    Date,
    DateTime,
    Email,
}

#[derive(Debug, PartialEq)]
pub enum DataValidation {
    TextLengthMin(u8),
    TextLengthMinMax(u8, u8),
}
