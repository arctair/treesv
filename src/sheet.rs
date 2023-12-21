use std::marker::PhantomData;
use std::str::FromStr;

pub(crate) struct Sheet<I> {
    pub(crate) schema: Schema,
    pub(crate) records: I,
}

impl<I: Iterator<Item=Vec<String>>> Sheet<I> {
    pub(crate) fn from(mut rows: I) -> Sheet<I> {
        Sheet {
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
        SchemaField {
            phantom_data: PhantomData {},
            position: self.position(field_name),
        }
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

impl<T: FromStr> SchemaField<T> {
    pub(crate) fn get(&self, row: &Vec<String>) -> Result<T, T::Err> {
        row[self.position].parse::<T>()
    }
}