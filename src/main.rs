use std::fs::File;
use treesv::schema_sheet::SchemaSheet;
use treesv::serialization::{DelimitedRowsWriter, FileToRowsExtension};

fn main() {
    println!("Hello, world!");
    let write_file = File::create("/tmp/balance_sheet").unwrap();
    let mut writer = DelimitedRowsWriter::new(write_file);

    let rows = File::open("/tmp/journal").unwrap().rows();
    let sheet = SchemaSheet::from(rows);
    let balance_sheet = sheet.balance_sheet();
    writer.write(balance_sheet).unwrap();
    println!("Goodbye, world!");
}
