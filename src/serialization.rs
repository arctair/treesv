use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};

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
        let mut writer = DelimitedRowsWriter::new(write_file);

        let rows = rows![
            ["token1", "token2"],
            ["token3"]
        ];

        writer.write(rows).unwrap();

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

struct DelimitedRowsWriter {
    buffered_writer: BufWriter<File>,
    column_separator: &'static str,
    row_separator: &'static str,
}

impl DelimitedRowsWriter {
    fn new(file: File) -> DelimitedRowsWriter {
        DelimitedRowsWriter {
            buffered_writer: BufWriter::new(file),
            column_separator: "\t",
            row_separator: "\n",
        }
    }

    fn write(&mut self, iterator: impl Iterator<Item=Vec<String>>) -> Result<(), Error> {
        for row in iterator {
            let line = format!("{}{}", row.join(self.column_separator), self.row_separator);
            self.buffered_writer.write(line.as_bytes())?;
        }
        self.buffered_writer.flush()
    }
}