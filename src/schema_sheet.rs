use std::marker::PhantomData;
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

pub(crate) trait SchemaFieldValue {
    fn value<T: FromStr>(&self, field: &SchemaField<T>) -> Result<T, T::Err>;
}

impl SchemaFieldValue for Vec<String> {
    fn value<T: FromStr>(&self, field: &SchemaField<T>) -> Result<T, T::Err> {
        self[field.position].parse::<T>()
    }
}
