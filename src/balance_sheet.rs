use std::collections::{BTreeMap, BTreeSet};
use rusty_money::{iso, Money, MoneyError};
use rusty_money::iso::Currency;
use crate::sheet::{Schema, Sheet};

pub struct BalanceSheet(pub Sheet);

pub struct Journal(pub Sheet);

impl From<Journal> for BalanceSheet {
    fn from(Journal(sheet): Journal) -> Self {
        let zero: Money<Currency> = Money::from_major(0, iso::USD);

        let mut rows = sheet.create_year_field("date", "year").rows();

        let Some(schema) = rows.next().map(Schema::from) else { todo!("no schema") };
        let selector = schema.selector(["year", "account_name", "debit_amount", "credit_amount"]);

        let mut balance_amounts = BTreeMap::new();
        for record in rows {
            let mut selection = selector(record);

            let year = selection.pop_front().unwrap();

            let mut account_name = selection.pop_front().unwrap();
            trim_mut(&mut account_name);

            let debit_amount = selection.pop_front().unwrap();
            let debit_amount = parse_money(debit_amount).unwrap();

            let credit_amount = selection.pop_front().unwrap();
            let credit_amount = parse_money(credit_amount).unwrap();

            let entry = balance_amounts
                .entry((year, account_name))
                .or_insert(zero.clone());
            *entry += debit_amount - credit_amount;
        }

        let mut years = BTreeSet::new();
        let mut account_names = BTreeSet::new();
        for (year, account_name) in balance_amounts.keys() {
            years.insert(year.clone());
            account_names.insert(account_name.clone());
        }

        let mut schema_vec = vec!["account_type".to_string(), "account_name".to_string()];
        for year in &years {
            schema_vec.insert(2, format!("balance_amount_{year}"));
        }

        let mut result = vec![schema_vec];
        for account_name in account_names {
            let account_type = "";
            let mut result_record = vec![account_type.to_string(), account_name.to_string()];
            let mut cumulative_balance_amount = zero.clone();
            for year in &years {
                let key = (year.to_string(), account_name.to_string());
                if let Some(balance_amount) = balance_amounts.remove(&key) {
                    cumulative_balance_amount += balance_amount;
                }
                result_record.insert(2, cumulative_balance_amount.to_string());
            }
            result.push(result_record);
        }

        BalanceSheet(Sheet::from(result))
    }
}

fn parse_money<'a>(value: String) -> Result<Money<'a, Currency>, MoneyError> {
    if value.is_empty() {
        return Ok(Money::from_major(0, iso::USD));
    }

    let value = value.trim();
    let value = value.trim_start_matches("$");
    let value = value.trim_start();

    Money::from_str(value, iso::USD)
}

fn trim_mut(string: &mut String) {
    while string.starts_with(" ") {
        string.remove(0);
    }
    while string.ends_with(" ") {
        string.pop();
    }
}