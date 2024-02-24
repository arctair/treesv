use treesv::balance_sheet::{BalanceSheet, Journal};
use treesv::sheet::Sheet;

#[test]
fn balance_sheet_from_journal() {
    let journal_sheet = Sheet::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["credit account", "1.00", "0.00"],
        vec!["debit account", "0.00", "1.00"],
        vec!["credit account", "1.00", "0.00"],
        vec!["debit account", "0.00", "1.00"],
    ]);

    let BalanceSheet(actual) = BalanceSheet::from(Journal(journal_sheet));
    let expected = Sheet::from(vec![
        vec!["account_name", "balance_amount"],
        vec!["credit account", "-$2.00"],
        vec!["debit account", "$2.00"],
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn empty_amount_is_zero() {
    let journal_sheet = Sheet::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["account", "", ""],
    ]);

    let BalanceSheet(actual) = BalanceSheet::from(Journal(journal_sheet));
    let expected = Sheet::from(vec![
        vec!["account_name", "balance_amount"],
        vec!["account", "$0"],
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn google_accounting_number_format() {
    let journal_sheet = Sheet::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["account", " $0.00 ", " $ 1.00 "],
    ]);

    let BalanceSheet(actual) = BalanceSheet::from(Journal(journal_sheet));
    let expected = Sheet::from(vec![
        vec!["account_name", "balance_amount"],
        vec!["account", "$1.00"],
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn ignore_account_name_outer_space() {
    let journal_sheet = Sheet::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec![" account ", "", ""],
    ]);

    let BalanceSheet(actual) = BalanceSheet::from(Journal(journal_sheet));
    let expected = Sheet::from(vec![
        vec!["account_name", "balance_amount"],
        vec!["account", "$0"],
    ]);

    assert_eq!(actual, expected);
}
