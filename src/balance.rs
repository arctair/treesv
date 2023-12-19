#[cfg(test)]
mod tests {
    use crate::{rows};
    use crate::balance::BalanceExtension;

    #[test]
    fn balance() {
        let balance: i32 = rows![
            ["debit"],
            ["125"],
            ["500"]
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
        iter.next().unwrap();
        iter.map(|row| row[0].parse::<i32>().expect("to have parsed debit value")).sum()
    }
}