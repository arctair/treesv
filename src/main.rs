use std::fs::File;
use std::io::{BufReader, BufWriter};
use treesv::schema_sheet::{SchemaSheet, Sheet};

fn main() {
    println!("Hello, world!");
    let reader = File::open("/tmp/journal").map(BufReader::new).unwrap();
    let writer = File::create("/tmp/balance_sheet").map(BufWriter::new).unwrap();

    let sheet = Sheet::from_reader(reader);
    let schema_sheet = SchemaSheet::from(sheet.rows);
    let balance_sheet = schema_sheet.balance_sheet();
    balance_sheet.write(writer).unwrap();
    println!("Goodbye, world!");
}
