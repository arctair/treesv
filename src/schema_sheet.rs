use std::num::{IntErrorKind, ParseIntError};

pub(crate) struct SchemaSheet<I> {
    pub(crate) schema: Schema,
    pub(crate) records: I,
}

impl<I: Iterator<Item=Vec<String>>> SchemaSheet<I> {
    pub(crate) fn from(mut rows: I) -> SchemaSheet<I> {
        SchemaSheet {
            schema: Schema { field_names: rows.next().unwrap() },
            records: rows,
        }
    }
}


pub(crate) struct Schema {
    field_names: Vec<String>,
}

impl Schema {
    pub(crate) fn field(&self, field_name: &str) -> SchemaField {
        SchemaField::from(self.position(field_name))
    }

    fn position(&self, field_name: &str) -> usize {
        self.field_names.iter()
            .position(|check_field_name| check_field_name == field_name)
            .expect("to have found field position by name")
    }
}

pub(crate) struct SchemaField {
    position: usize,
}

impl SchemaField {
    fn from(position: usize) -> SchemaField {
        SchemaField { position }
    }
}

pub(crate) trait TextSchemaField {
    fn get_text(&self, field: &SchemaField) -> Option<&String>;
}

impl TextSchemaField for Vec<String> {
    fn get_text(&self, field: &SchemaField) -> Option<&String> {
        self.get(field.position)
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_sheet::{CurrencySchemaField, SchemaField};

    #[test]
    fn parse_empty_as_zero() {
        let field = SchemaField::from(0);
        let row = vec!["".to_string()];
        assert_eq!(row.get_currency(&field), Ok(0))
    }
}

pub(crate) trait CurrencySchemaField {
    fn get_currency(&self, field: &SchemaField) -> Result<i32, ParseIntError>;
}

impl CurrencySchemaField for Vec<String> {
    fn get_currency(&self, field: &SchemaField) -> Result<i32, ParseIntError> {
        match self[field.position].parse::<i32>() {
            Err(error) if *error.kind() == IntErrorKind::Empty => Ok(0),
            result => result
        }
    }
}
