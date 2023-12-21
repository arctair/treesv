use std::collections::BTreeMap;
use currency_rs::Currency;
use crate::schema_sheet::{TextSchemaField, SchemaSheet, CurrencySchemaField};

#[cfg(test)]
mod tests {
    use crate::{assert_rows_eq, sheet};
    use crate::schema_sheet::SchemaSheet;

    #[test]
    fn balance() {
        let balance_sheet = sheet![
            ["account", "debit"],
            ["assets", ""],
            ["assets", " $ 125.00"],
            ["assets", " $ 500.00"],
            ["expenses", " $ 300.00"]
        ].balance_sheet();

        assert_rows_eq!(
            balance_sheet,
            ["account", "balance"],
            ["assets", "625.00"],
            ["expenses", "300.00"]
        );
    }
}

impl<I: Iterator<Item=Vec<String>>> SchemaSheet<I> {
    pub fn balance_sheet(self) -> Box<dyn Iterator<Item=Vec<String>>> {
        let mut balances = BTreeMap::new();
        let account_field = self.schema.field("account");
        let debit_field = self.schema.field("debit");

        for record in self.records {
            let account_name = record.get_text(&account_field).expect("to have account name");
            let debit_value = record.get_currency(&debit_field).expect("to have parsed debit field value");
            let balance = balances.entry(account_name.to_string()).or_insert_with(|| Currency::new_float(0f64, None));
            *balance += debit_value;
        }

        let mut rows = Vec::new();
        rows.push(vec!["account".to_string(), "balance".to_string()]);

        for (account_name, balance) in balances {
            rows.push(vec![account_name, balance.to_string()]);
        }

        Box::new(rows.into_iter())
    }
}
