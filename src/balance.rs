use std::marker::PhantomData;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use crate::{rows};
    use crate::balance::BalanceExtension;

    #[test]
    fn balance() {
        let balance: i32 = rows![
            ["", "debit"],
            ["", "125"],
            ["", "500"]
        ].balance::<Vec<String>>();

        assert_eq!(balance, 625);
    }
}

trait BalanceExtension: Iterator {
    fn balance<B>(self) -> i32 where B: Balance<Self::Item>, Self: Sized {
        B::balance(self)
    }
}

impl<I: Iterator> BalanceExtension for I {}

trait Balance<Item = Self> {
    fn balance<I>(iter: I) -> i32 where I: Iterator<Item=Item>;
}

impl Balance for Vec<String> {
    fn balance<I>(mut iter: I) -> i32 where I: Iterator<Item=Self> {
        let schema = Schema { field_names: iter.next().unwrap() };
        let field = schema.field::<i32>("debit");
        iter.map(|row| field.get(&row).unwrap()).sum()
    }
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