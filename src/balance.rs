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

trait Balance<A = Self> {
    fn balance<I>(iter: I) -> i32 where I: Iterator<Item=A>;
}

impl Balance for Vec<String> {
    fn balance<I>(mut iter: I) -> i32 where I: Iterator<Item=Self> {
        let schema = Schema { field_names: iter.next().unwrap() };
        let position = schema.position("debit");
        iter.map(|row| row[position].parse::<i32>().expect("to have parsed debit value")).sum()
    }
}

struct Schema {
    field_names: Vec<String>,
}

impl Schema {
    fn position(self, field_name: &str) -> usize {
        self.field_names.iter()
            .position(|check_field_name| check_field_name == field_name)
            .expect("to have found field position by name")
    }
}