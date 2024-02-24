use std::collections::{BTreeMap};
use rusty_money::{iso, Money};
use rusty_money::iso::Currency;
use crate::sheet::Sheet;

pub struct Journal(pub Sheet);

#[derive(Debug, PartialEq)]
pub struct BalanceSheet<'a> {
    balance_amount_by_account_name: BTreeMap<String, Money<'a, Currency>>,
}

impl From<Journal> for BalanceSheet<'_> {
    fn from(Journal(sheet): Journal) -> Self {
        let mut balance_amount_by_account_name = BTreeMap::new();
        let mut rows = sheet.rows();

        let Some(schema) = rows.next() else { todo!("no schema") };
        let Some(account_name_index) = schema.iter().position(|field_name| field_name == "account_name") else { todo!("no schema name account_name in {:?}", schema) };
        let Some(debit_amount_index) = schema.iter().position(|field_name| field_name == "debit_amount") else { todo!("no schema name debit_amount in {:?}", schema) };
        let Some(credit_amount_index) = schema.iter().position(|field_name| field_name == "credit_amount") else { todo!("no schema name credit_amount in {:?}", schema) };

        for mut record in rows {
            let debit_amount = parse_money(&record[debit_amount_index]);
            let credit_amount = parse_money(&record[credit_amount_index]);
            let account_name = record.remove(account_name_index);
            let entry = balance_amount_by_account_name
                .entry(account_name)
                .or_insert(Money::from_major(0, iso::USD));
            *entry += debit_amount - credit_amount;
        }

        Self { balance_amount_by_account_name }
    }
}

fn parse_money<'a>(value: &str) -> Money<'a, Currency> {
    if value.is_empty() {
        return Money::from_major(0, iso::USD);
    }

    let value = value.trim();
    let value = value.trim_start_matches("$");
    let value = value.trim_start();

    match Money::from_str(value, iso::USD) {
        Ok(debit_amount) => debit_amount,
        Err(error) => todo!("parse money <{}>: {error}", value)
    }
}

impl BalanceSheet<'_> {
    pub fn to_sheet(self) -> Sheet {
        let mut result = vec![];
        result.push(vec![String::from("account_name"), String::from("balance_amount")]);
        for (account_name, balance_amount) in self.balance_amount_by_account_name {
            result.push(vec![account_name, balance_amount.to_string()]);
        }
        Sheet::from(result)
    }
}