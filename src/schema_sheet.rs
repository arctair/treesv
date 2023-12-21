use std::marker::PhantomData;
use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;

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
    pub(crate) fn field<T>(&self, field_name: &str) -> SchemaField<T> {
        SchemaField::from(self.position(field_name))
    }

    fn position(&self, field_name: &str) -> usize {
        self.field_names.iter()
            .position(|check_field_name| check_field_name == field_name)
            .expect("to have found field position by name")
    }
}

pub(crate) struct SchemaField<T> {
    phantom_data: PhantomData<T>,
    position: usize,
}

impl<T> SchemaField<T> {
    fn from(position: usize) -> SchemaField<T> {
        SchemaField {
            phantom_data: PhantomData {},
            position,
        }
    }
}

pub(crate) trait TextSchemaField {
    fn text<T: FromStr>(&self, field: &SchemaField<T>) -> Result<T, T::Err>;
}

impl TextSchemaField for Vec<String> {
    fn text<T: FromStr>(&self, field: &SchemaField<T>) -> Result<T, T::Err> {
        self[field.position].parse::<T>()
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_sheet::{CurrencySchemaField, SchemaField};

    #[test]
    fn parse_empty_as_zero() {
        let field = SchemaField::<i32>::from(0);
        let row = vec!["".to_string()];
        assert_eq!(row.currency(&field), Ok(0))
    }
}

pub(crate) trait CurrencySchemaField {
    fn currency(&self, field: &SchemaField<i32>) -> Result<i32, ParseIntError>;
}

impl CurrencySchemaField for Vec<String> {
    fn currency(&self, field: &SchemaField<i32>) -> Result<i32, ParseIntError> {
        match self[field.position].parse::<i32>() {
            Err(error) if *error.kind() == IntErrorKind::Empty => Ok(0),
            result => result
        }
    }
}
