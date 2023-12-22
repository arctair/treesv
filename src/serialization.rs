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

        let sheet = Sheet::from_reader(BufReader::new(File::open(&path).unwrap()));

        assert_rows_eq!(
            sheet.rows,
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

impl Sheet<()> {
    pub fn from_reader(buffered_reader: BufReader<File>) -> Sheet<impl Iterator<Item=Vec<String>>> {
        let rows = buffered_reader.lines()
            .map(Result::unwrap)
            .map(|line| line.split("\t").map(str::to_string).collect::<Vec<String>>());
        Sheet::from(rows)
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