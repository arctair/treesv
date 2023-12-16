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
        fs::write("testfile", "token1\ttoken2").expect("Failed to write file");
        let file = File::open("testfile")?;
        let treesv = TreeSV::new(file);
        let row = treesv.read_row().expect("Failed to read row");
        assert_eq!(row, vec!["token1", "token2"]);
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