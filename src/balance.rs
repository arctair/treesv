use std::collections::BTreeMap;
use crate::schema_sheet::{SchemaFieldValue, SchemaSheet};

#[cfg(test)]
mod tests {
    use crate::{assert_rows_eq, sheet};
    use crate::schema_sheet::SchemaSheet;

    #[test]
    fn balance() {
        let balance_sheet = sheet![
            ["account", "debit"],
            ["assets", "125"],
            ["assets", "500"],
            ["expenses", "300"]
        ].balance_sheet();

        assert_rows_eq!(
            balance_sheet,
            ["account", "balance"],
            ["assets", "625"],
            ["expenses", "300"]
        );
    }
}

impl<I: Iterator<Item=Vec<String>>> SchemaSheet<I> {
    fn balance_sheet(self) -> Box<dyn Iterator<Item=Vec<String>>> {
        let mut balances = BTreeMap::new();
        let account_field = self.schema.field::<String>("account");
        let debit_field = self.schema.field::<i32>("debit");

        for record in self.records {
            let account_name = record.value(&account_field).expect("to have fetched account name field value");
            let debit_value = record.value(&debit_field).expect("to have parsed debit field value");
            *balances.entry(account_name).or_insert(0) += debit_value;
        }

        let mut rows = Vec::new();
        rows.push(vec!["account".to_string(), "balance".to_string()]);

        for (account_name, balance) in balances {
            rows.push(vec![account_name, balance.to_string()]);
        }

        Box::new(rows.into_iter())
    }
}
