use indoc::indoc;
use treesv::sheet::Sheet;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn sheet_from_string() {
    let input = String::from(indoc! {"
        account_name\tcredit_amount\tdebit_amount
        credit account\t1.00\t0.00
        debit account\t0.00\t1.00"});

    let actual = Sheet::from(input);
    let expected = Sheet::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["credit account", "1.00", "0.00"],
        vec!["debit account", "0.00", "1.00"],
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn sheet_from_file() {
    let input = String::from(indoc! {"
        account_name\tcredit_amount\tdebit_amount
        credit account\t1.00\t0.00
        debit account\t0.00\t1.00"});

    let mut file = NamedTempFile::new().unwrap();
    writeln!(&mut file, "{}", input).unwrap();

    let actual = Sheet::from(file.reopen().unwrap());
    let expected = Sheet::from(vec![
        vec!["account_name", "credit_amount", "debit_amount"],
        vec!["credit account", "1.00", "0.00"],
        vec!["debit account", "0.00", "1.00"],
    ]);

    assert_eq!(actual, expected);
}
