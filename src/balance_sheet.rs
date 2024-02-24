use std::collections::{BTreeMap};
use rusty_money::{iso, Money, MoneyError};
use rusty_money::iso::Currency;
use crate::sheet::Sheet;

pub struct BalanceSheet(pub Sheet);

pub struct Journal(pub Sheet);

impl From<Journal> for BalanceSheet {
    fn from(Journal(sheet): Journal) -> Self {
        let mut balance_amount_by_account_name = BTreeMap::new();
        let mut rows = sheet.rows();

        let Some(schema) = rows.next() else { todo!("no schema") };
        let Some(account_name_index) = schema.iter().position(|field_name| field_name == "account_name") else { todo!("no schema name account_name in {:?}", schema) };
        let Some(debit_amount_index) = schema.iter().position(|field_name| field_name == "debit_amount") else { todo!("no schema name debit_amount in {:?}", schema) };
        let Some(credit_amount_index) = schema.iter().position(|field_name| field_name == "credit_amount") else { todo!("no schema name credit_amount in {:?}", schema) };

        for mut record in rows {
            let debit_amount = parse_money(&record[debit_amount_index]).unwrap();
            let credit_amount = parse_money(&record[credit_amount_index]).unwrap();
            let mut account_name = record.remove(account_name_index);
            trim_mut(&mut account_name);
            let entry = balance_amount_by_account_name
                .entry(account_name)
                .or_insert(Money::from_major(0, iso::USD));
            *entry += debit_amount - credit_amount;
        }

        let mut result = vec![];

        result.push(vec![String::from("account_name"), String::from("balance_amount_2024")]);
        for (account_name, balance_amount) in balance_amount_by_account_name {
            result.push(vec![account_name, balance_amount.to_string()]);
        }

        BalanceSheet(Sheet::from(result))
    }
}

fn parse_money<'a>(value: &str) -> Result<Money<'a, Currency>, MoneyError> {
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