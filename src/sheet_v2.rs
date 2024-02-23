#[derive(Debug, PartialEq)]
pub struct SheetV2 {
    rows: Vec<Vec<String>>,
}

impl From<String> for SheetV2 {
    fn from(string: String) -> Self {
        let empty = vec![""];
        let mut rows = Vec::new();
        for line in string.lines() {
            let record = line.split("\t").collect::<Vec<_>>();
            if record != empty {
                rows.push(record);
            }
        }

        Self::from(rows)
    }
}

impl From<Vec<Vec<&str>>> for SheetV2 {
    fn from(rows_references: Vec<Vec<&str>>) -> Self {
        let rows = rows_references.iter()
            .map(|row| row.iter().map(|&value| value.to_string()).collect())
            .collect();
        Self { rows }
    }
}

impl From<Vec<Vec<String>>> for SheetV2 {
    fn from(rows: Vec<Vec<String>>) -> Self {
        Self { rows }
    }
}

impl SheetV2 {
    pub fn rows(&self) -> impl Iterator<Item=Vec<&str>> {
        self.rows.iter().map(|row| row.iter().map(|s| s as &str).collect::<Vec<&str>>())
    }
}