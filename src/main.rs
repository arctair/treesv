use std::fs::File;
use std::io::{BufReader, BufWriter};
use treesv::schema_sheet::{SchemaSheet, Sheet};

fn main() {
    println!("Hello, world!");
    let write_file = File::create("/tmp/balance_sheet").unwrap();
    let read_file = File::open("/tmp/journal").unwrap();

    let sheet = Sheet::from_reader(BufReader::new(read_file));
    let schema_sheet = SchemaSheet::from(sheet.rows);
    let balance_sheet = schema_sheet.balance_sheet();
    balance_sheet.write(BufWriter::new(write_file)).unwrap();
    println!("Goodbye, world!");
}
