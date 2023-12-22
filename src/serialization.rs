use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use crate::schema_sheet::Sheet;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use crate::{assert_rows_eq, rows};
    use crate::temporary_directory::*;
    use super::*;

    #[test]
    fn deserialize_delimited_rows() {
        let mut temporary_directory = TemporaryDirectory::new();
        let path = temporary_directory.get_child_path();

        fs::write(&path, "token1\ttoken2\ntoken3").unwrap();

        assert_rows_eq!(
            File::open(&path).unwrap().rows(),
            ["token1", "token2"],
            ["token3"]
        );
    }

    #[test]
    fn serialize_delimited_rows() {
        let mut temporary_directory = TemporaryDirectory::new();
        let write_path = temporary_directory.get_child_path();
        let write_file = File::create(&write_path).unwrap();

        let rows = rows![
            ["token1", "token2"],
            ["token3"]
        ];

        let sheet = Sheet::from(rows);

        sheet.write(BufWriter::new(write_file)).unwrap();

        assert_eq!(fs::read_to_string(write_path).unwrap(), "token1\ttoken2\ntoken3\n");
    }
}

pub trait FileToRowsExtension {
    fn rows(self) -> Box<dyn Iterator<Item=Vec<String>>>;
}

impl FileToRowsExtension for File {
    fn rows(self) -> Box<dyn Iterator<Item=Vec<String>>> {
        Box::new(DelimitedRowsReader::new(BufReader::new(self)).rows())
    }
}

struct DelimitedRowsReader {
    buffered_reader: BufReader<File>,
}

impl DelimitedRowsReader {
    fn new(buffered_reader: BufReader<File>) -> DelimitedRowsReader {
        DelimitedRowsReader { buffered_reader }
    }

    fn rows(self) -> impl Iterator<Item=Vec<String>> {
        self.buffered_reader.lines().map(Result::unwrap).map(|line| {
            line.split("\t").map(str::to_string).collect()
        })
    }
}

impl<I: Iterator<Item=Vec<String>>> Sheet<I> {
    pub fn write(self, mut buffered_writer: BufWriter<File>) -> Result<(), Error> {
        let column_separator: &'static str = "\t";
        let row_separator: &'static str = "\n";
        for row in self.rows {
            let line = format!("{}{}", row.join(column_separator), row_separator);
            buffered_writer.write(line.as_bytes())?;
        }
        buffered_writer.flush()
    }
}