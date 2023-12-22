use std::collections::BTreeMap;
use std::vec::IntoIter;
use currency_rs::Currency;
use crate::schema_sheet::{TextSchemaField, SchemaSheet, CurrencySchemaField, Sheet};

#[cfg(test)]
mod tests {
    use crate::{assert_rows_eq, sheet};
    use crate::schema_sheet::SchemaSheet;

    #[test]
    fn balance() {
        let balance_sheet = sheet![
            ["account", "credit", "debit"],
            ["assets", "", ""],
            ["assets", " $ 125.00", ""],
            ["assets", "", " $ 500.00"],
            ["expenses", "", " $ 300.00"]
        ].balance_sheet();

        assert_rows_eq!(
            balance_sheet.rows,
            ["account", "balance"],
            ["assets", "375.00"],
            ["expenses", "300.00"]
        );
    }

    #[test]
    fn exclude_empty_account_zero_balance() {
        let balance_sheet = sheet![
            ["account", "credit", "debit"],
            ["", "", ""]
        ].balance_sheet();

        assert_rows_eq!(
            balance_sheet.rows,
            ["account", "balance"]
        );
    }

    #[test]
    fn include_empty_account_non_zero_balance() {
        let balance_sheet = sheet![
            ["account", "credit", "debit"],
            ["", "", " $ 1.00"]
        ].balance_sheet();

        assert_rows_eq!(
            balance_sheet.rows,
            ["account", "balance"],
            ["", "1.00"]
        );
    }
}

impl<I: Iterator<Item=Vec<String>>> SchemaSheet<I> {
    pub fn balance_sheet(self) -> Sheet<IntoIter<Vec<String>>> {
        let mut balances = BTreeMap::new();
        let account_field = self.schema.field("account");
        let credit_field = self.schema.field("credit");
        let debit_field = self.schema.field("debit");

        for record in self.records {
            let account_name = record.get_text(&account_field).expect("to have account name");
            let credit_value = record.get_currency(&credit_field).expect("to have parsed credit field value");
            let debit_value = record.get_currency(&debit_field).expect("to have parsed debit field value");
            let balance = balances.entry(account_name.to_string()).or_insert_with(|| Currency::new_float(0f64, None));
            *balance += debit_value - credit_value;
        }

        let mut rows = Vec::new();
        rows.push(vec!["account".to_string(), "balance".to_string()]);

        for (account_name, balance) in balances {
            if !account_name.is_empty() || balance.int_value() != 0f64 {
                rows.push(vec![account_name, balance.to_string()]);
            }
        }

        Sheet::from(rows.into_iter())
    }
}
