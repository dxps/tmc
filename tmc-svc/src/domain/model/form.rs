use super::{DataItemDef, DataValidation};

#[derive(Debug)]
pub struct Form {
    fields: Vec<FormField>,
}

#[derive(Debug)]
pub struct FormField {
    /// The definition of this field.
    field_def: DataItemDef,
    /// The order number in the UI.
    form_order_number: u8,
    /// Tells if this field is mandatory.
    is_mandatory: bool,
    /// Specify any potential data validation rules.
    validations: Vec<DataValidation>,
}

#[cfg(test)]
mod tests {
    use crate::domain::model::DataItemDef;

    use super::{Form, FormField};

    #[test]
    fn simple() {
        let name = "First name";
        let is_mandatory = true;
        let attr_def = DataItemDef::new_text(name.into(), None);
        let field1 = FormField {
            field_def: attr_def,
            form_order_number: 1,
            is_mandatory,
            validations: vec![],
        };
        let form = Form {
            fields: vec![field1],
        };
        assert_eq!(name, form.fields[0].field_def.name);
        assert_eq!(is_mandatory, form.fields[0].is_mandatory);
        assert_eq!(0, form.fields[0].validations.len());
    }
}
