use std::marker::PhantomData;
use std::str::FromStr;
use std::vec::IntoIter;

#[cfg(test)]
mod tests {
    use crate::{rows};
    use crate::balance::{balance_sheet};

    #[test]
    fn balance() {
        let rows = rows![
            ["", "debit"],
            ["", "125"],
            ["", "500"]
        ];
        let balance = balance_sheet(rows);

        assert_eq!(balance, 625);
    }
}

fn balance_sheet(mut rows: IntoIter<Vec<String>>) -> i32 {
    let schema = Schema { field_names: rows.next().unwrap() };
    let field = schema.field::<i32>("debit");
    rows.map(|row| field.get(&row).unwrap()).sum()
}

struct Schema {
    field_names: Vec<String>,
}

impl Schema {
    fn field<T>(&self, field_name: &str) -> SchemaField<T> {
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


struct SchemaField<T> {
    phantom_data: PhantomData<T>,
    position: usize,
}

impl<T: FromStr> SchemaField<T> {
    fn get(&self, row: &Vec<String>) -> Result<T, T::Err> {
        row[self.position].parse::<T>()
    }
}