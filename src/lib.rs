use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Error;
    use super::*;

    #[test]
    fn echo_line_from_file() -> Result<(), Error> {
        fs::write("testfile", "token1\ttoken2\ntoken3").expect("Failed to write file");
        let file = File::open("testfile")?;
        let treesv = TreeSV::new(file);

        let rows = treesv.read_rows().map(|row| row.expect("Failed to read row")).collect::<Vec<Vec<String>>>();
        assert_eq!(rows, vec![vec!["token1", "token2"], vec!["token3"]]);
        Ok(())
    }
}

struct TreeSV {
    buffered_reader: BufReader<File>,
}

impl TreeSV {
    fn new(file: File) -> TreeSV {
        let buffered_reader = BufReader::new(file);
        TreeSV { buffered_reader }
    }

    fn read_rows(self) -> impl Iterator<Item=Result<Vec<String>, Error>> {
        self.buffered_reader.lines().map(|line| {
            match line {
                Err(string) => Err(string),
                Ok(line_present) => Ok(line_present.split("\t").map(str::to_string).collect())
            }
        })
    }

    fn read_row(self) -> Result<Vec<String>, Error> {
        let line = self.read_line();
        let row = line?.split("\t").map(str::to_string).collect();
        Ok(row)
    }

    fn read_line(mut self) -> Result<String, Error> {
        let mut line = String::new();
        self.buffered_reader.read_line(&mut line)?;
        Ok(line)
    }
}