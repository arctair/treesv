use indoc::indoc;
use treesv::sheet_v2::SheetV2;

#[test]
fn sheet_from_string() {
    let input = String::from(indoc! {"
        account_name\tcredit_amount\tdebit_amount
        credit account\t1.00\t0.00
        debit account\t0.00\t1.00"});

    let actual = SheetV2::from(input);
    let expected = SheetV2::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["credit account", "1.00", "0.00"],
        vec!["debit account", "0.00", "1.00"],
    ]);

    assert_eq!(actual, expected);
}
