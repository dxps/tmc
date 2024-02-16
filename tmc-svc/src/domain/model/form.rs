use super::{AttributeDefinition, DataValidation};

#[derive(Debug)]
pub struct Form {
    fields: Vec<FormField>,
}

#[derive(Debug)]
pub struct FormField {
    field_def: AttributeDefinition,
    is_mandatory: bool,
    validation: Option<DataValidation>,
}

#[cfg(test)]
mod tests {
    use crate::domain::model::AttributeDefinition;

    use super::{Form, FormField};

    #[test]
    fn simple() {
        let name = "First name";
        let is_mandatory = true;
        let attr_def = AttributeDefinition::new_text(name.into(), None);
        let field1 = FormField {
            field_def: attr_def,
            is_mandatory,
            validation: None,
        };
        let form = Form {
            fields: vec![field1],
        };
        assert_eq!(name, form.fields[0].field_def.name);
        assert_eq!(is_mandatory, form.fields[0].is_mandatory);
        assert_eq!(None, form.fields[0].validation);
    }
}
