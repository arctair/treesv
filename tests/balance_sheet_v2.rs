use indoc::indoc;
use treesv::balance_sheet_v2::{BalanceSheet, Journal};

#[test]
fn balance_sheet_v2() {
    let input = Journal::from(String::from(indoc! {"
        account_name\tcredit_amount\tdebit_amount
        credit account\t1.00\t0.00
        debit account\t0.00\t1.00"}));

    let actual = BalanceSheet::from(input).to_string();
    let expected = String::from(String::from(indoc! {"
        account_name\tbalance_amount
        credit account\t-$1.00
        debit account\t$1.00
    "}));

    assert_eq!(actual, expected);
}