use std::collections::{BTreeMap};
use rusty_money::{iso, Money};
use rusty_money::iso::Currency;
use crate::sheet_v2::SheetV2;

pub struct Journal(pub SheetV2);

#[derive(Debug, PartialEq)]
pub struct BalanceSheet<'a> {
    balance_amount_by_account_name: BTreeMap<String, Money<'a, Currency>>,
}

impl From<Journal> for BalanceSheet<'_> {
    fn from(journal: Journal) -> Self {
        let mut balance_amount_by_account_name = BTreeMap::new();
        let mut rows = journal.0.rows();

        let Some(schema) = rows.next() else { todo!("no schema") };
        let Some(account_name_index) = schema.iter().position(|&field_name| field_name == "account_name") else { todo!("no schema name account_name in {:?}", schema) };
        let Some(debit_amount_index) = schema.iter().position(|&field_name| field_name == "debit_amount") else { todo!("no schema name debit_amount in {:?}", schema) };
        let Some(credit_amount_index) = schema.iter().position(|&field_name| field_name == "credit_amount") else { todo!("no schema name credit_amount in {:?}", schema) };

        for record in rows {
            let account_name = record[account_name_index];
            let debit_amount = parse_money(record[debit_amount_index]);
            let credit_amount = parse_money(record[credit_amount_index]);
            balance_amount_by_account_name.insert(String::from(account_name), debit_amount - credit_amount);
        }

        Self { balance_amount_by_account_name }
    }
}

fn parse_money<'a>(value: &str) -> Money<'a, Currency> {
    match Money::from_str(&value, iso::USD) {
        Ok(debit_amount) => debit_amount,
        Err(error) => todo!("parse money <{}>: {error}", value)
    }
}

impl BalanceSheet<'_> {
    pub fn to_sheet(&self) -> SheetV2 {
        let mut result = vec![];
        result.push(vec![String::from("account_name"), String::from("balance_amount")]);
        for (account_name, balance_amount) in &self.balance_amount_by_account_name {
            result.push(vec![account_name.to_string(), balance_amount.to_string()]);
        }
        SheetV2::from(result)
    }
}