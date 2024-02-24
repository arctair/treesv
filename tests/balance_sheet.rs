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

    let actual = BalanceSheet::from(Journal(journal_sheet)).to_sheet();
    let expected = Sheet::from(vec![
        vec!["account_name", "balance_amount"],
        vec!["credit account", "-$2.00"],
        vec!["debit account", "$2.00"],
    ]);

    assert_eq!(actual, expected);
}