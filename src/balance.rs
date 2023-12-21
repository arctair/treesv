use crate::schema_sheet::{SchemaFieldValue, SchemaSheet};

#[cfg(test)]
mod tests {
    use crate::{assert_rows_eq, sheet};
    use crate::schema_sheet::SchemaSheet;

    #[test]
    fn balance() {
        let balance_sheet = sheet![
            ["", "debit"],
            ["", "125"],
            ["", "500"]
        ].balance_sheet();

        assert_rows_eq!(
            balance_sheet,
            ["625"]
        );
    }
}

impl<I: Iterator<Item=Vec<String>>> SchemaSheet<I> {
    fn balance_sheet(self) -> Box<dyn Iterator<Item=Vec<String>>> {
        let debit_field = self.schema.field::<i32>("debit");
        
        let mut balance = 0;
        for record in self.records {
            let debit_value = record.value(&debit_field).expect("to have parsed debit field value");
            balance += debit_value;
        }

        Box::new(vec![vec![balance.to_string()]].into_iter())
    }
}
