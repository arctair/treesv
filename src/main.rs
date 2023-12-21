use std::fs::File;
use treesv::serialization::FileToRowsExtension;

fn main() {
    println!("Hello, world!");
    let rows = File::open("/tmp/journal").unwrap().rows();

    let write_file = File::create("/tmp/balance_sheet").unwrap();
    let mut writer = treesv::serialization::DelimitedRowsWriter::new(write_file);
    writer.write(rows).unwrap();
}
