use std::fs::File;
use std::io::BufWriter;
use treesv::schema_sheet::SchemaSheet;
use treesv::serialization::{FileToRowsExtension};

fn main() {
    println!("Hello, world!");
    let write_file = File::create("/tmp/balance_sheet").unwrap();

    let rows = File::open("/tmp/journal").unwrap().rows();
    let sheet = SchemaSheet::from(rows);
    let balance_sheet = sheet.balance_sheet();
    balance_sheet.write(BufWriter::new(write_file)).unwrap();
    println!("Goodbye, world!");
}
