use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[cfg(test)]
mod temporary_directory;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use crate::temporary_directory::*;
    use super::*;

    #[test]
    fn echo_line_from_file() {
        let mut temporary_directory = TemporaryDirectory::new();
        let path = temporary_directory.get_child_path();

        fs::write(&path, "token1\ttoken2\ntoken3").unwrap();

        let file = File::open(path).unwrap();
        let treesv = TreeSVReader::new(file);

        let rows = treesv.rows()
            .map(Result::unwrap)
            .collect::<Vec<Vec<String>>>();

        assert_eq!(rows, vec![vec!["token1", "token2"], vec!["token3"]]);
    }
}


struct TreeSVReader {
    buffered_reader: BufReader<File>,
}

impl TreeSVReader {
    fn new(file: File) -> TreeSVReader {
        TreeSVReader { buffered_reader: BufReader::new(file) }
    }

    fn rows(self) -> impl Iterator<Item=Result<Vec<String>, Error>> {
        self.buffered_reader.lines().map(|line| {
            match line {
                Err(string) => Err(string),
                Ok(line_present) => Ok(line_present.split("\t").map(str::to_string).collect())
            }
        })
    }
}



