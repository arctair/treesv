use treesv::balance_sheet_v2::{BalanceSheet, Journal};
use treesv::sheet_v2::SheetV2;

#[test]
fn balance_sheet_from_journal() {
    let journal_sheet = SheetV2::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["credit account", "1.00", "0.00"],
        vec!["debit account", "0.00", "1.00"],
    ]);

    let actual = BalanceSheet::from(Journal(journal_sheet)).to_sheet();
    let expected = SheetV2::from(vec![
        vec!["account_name", "balance_amount"],
        vec!["credit account", "-$1.00"],
        vec!["debit account", "$1.00"],
    ]);

    assert_eq!(actual, expected);
}
