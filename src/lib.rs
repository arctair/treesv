use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};

#[cfg(test)]
mod temporary_directory;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use crate::temporary_directory::*;
    use super::*;

    #[test]
    fn deserialize_delimited_rows() {
        let mut temporary_directory = TemporaryDirectory::new();
        let path = temporary_directory.get_child_path();

        fs::write(&path, "token1\ttoken2\ntoken3").unwrap();

        let file = File::open(path).unwrap();
        let reader = DelimitedRowsReader::new(file);

        let rows = reader.rows().collect::<Vec<Vec<String>>>();

        assert_eq!(rows, vec![vec!["token1", "token2"], vec!["token3"]]);
    }

    #[test]
    fn serialize_delimited_rows() {
        let mut temporary_directory = TemporaryDirectory::new();
        let read_path = temporary_directory.get_child_path();
        let write_path = temporary_directory.get_child_path();

        let contents = "token1\ttoken2\ntoken3\n";
        fs::write(&read_path, contents).unwrap();

        let read_file = File::open(read_path).unwrap();
        let reader = DelimitedRowsReader::new(read_file);

        let write_file = File::create(&write_path).unwrap();
        let mut writer = DelimitedRowsWriter::new(write_file);

        writer.write(reader.rows()).expect("rows to be written");

        assert_eq!(fs::read_to_string(write_path).unwrap(), contents);
    }
}


struct DelimitedRowsReader {
    buffered_reader: BufReader<File>,
}

impl DelimitedRowsReader {
    fn new(file: File) -> DelimitedRowsReader {
        DelimitedRowsReader { buffered_reader: BufReader::new(file) }
    }

    fn rows(self) -> impl Iterator<Item=Vec<String>> {
        self.buffered_reader.lines().map(Result::unwrap).map(|line| {
            line.split("\t").map(str::to_string).collect()
        })
    }
}

struct DelimitedRowsWriter {
    buffered_writer: BufWriter<File>,
}

impl DelimitedRowsWriter {
    fn new(file: File) -> DelimitedRowsWriter {
        DelimitedRowsWriter { buffered_writer: BufWriter::new(file) }
    }

    fn write(&mut self, iterator: impl Iterator<Item=Vec<String>>) -> Result<(), Error> {
        for row in iterator {
            let line = format!("{}\n", row.join("\t"));
            self.buffered_writer.write_all(line.as_bytes())?
        }
        self.buffered_writer.flush()
    }
}
