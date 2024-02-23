use std::collections::{BTreeMap};
use std::fmt::Write;
use rusty_money::{iso, Money};
use rusty_money::iso::Currency;

pub struct Journal {
    text: String,
}

impl From<String> for Journal {
    fn from(text: String) -> Self {
        Self { text }
    }
}

#[derive(Debug, PartialEq)]
pub struct BalanceSheet<'a> {
    balance_amount_by_account_name: BTreeMap<String, Money<'a, Currency>>,
}

impl From<Journal> for BalanceSheet<'_> {
    fn from(journal: Journal) -> Self {
        let mut balance_amount_by_account_name = BTreeMap::new();
        let mut lines = journal.text.lines();

        let empty = vec![""];
        let mut schema = empty.clone();
        while schema == empty {
            let Some(line) = lines.next() else { todo!("no schema") };
            schema = line.split("\t").collect::<Vec<_>>();
        }
        let Some(account_name_index) = schema.iter().position(|&field_name| field_name == "account_name") else { todo!("no schema name account_name in {:?}", schema) };
        let Some(debit_amount_index) = schema.iter().position(|&field_name| field_name == "debit_amount") else { todo!("no schema name debit_amount in {:?}", schema) };
        let Some(credit_amount_index) = schema.iter().position(|&field_name| field_name == "credit_amount") else { todo!("no schema name credit_amount in {:?}", schema) };

        for line in lines {
            let record = line.split("\t").collect::<Vec<_>>();
            if record == empty { continue; }

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

impl ToString for BalanceSheet<'_> {
    fn to_string(&self) -> String {
        let mut result = String::new();
        write!(&mut result, "account_name\tbalance_amount\n").unwrap();
        for (account_name, balance_amount) in &self.balance_amount_by_account_name {
            write!(&mut result, "{account_name}\t{balance_amount}\n").unwrap();
        }
        result
    }
}