use crate::sheet::Sheet;

#[cfg(test)]
mod tests {
    use crate::sheet;
    use crate::sheet::Sheet;

    #[test]
    fn balance() {
        let balance = sheet![
            ["", "debit"],
            ["", "125"],
            ["", "500"]
        ].balance_sheet();

        assert_eq!(balance, vec![vec!["625"]]);
    }
}

impl<I: Iterator<Item=Vec<String>>> Sheet<I> {
    fn balance_sheet(self) -> Vec<Vec<String>> {
        let field = self.schema.field::<i32>("debit");
        let balance: i32 = self.records.map(|row| field.get(&row).unwrap()).sum();
        vec![vec![balance.to_string()]]
    }
}
