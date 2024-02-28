use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub struct Sheet {
    rows: Vec<Vec<String>>,
}

impl From<String> for Sheet {
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

impl From<File> for Sheet {
    fn from(file: File) -> Self {
        let reader = BufReader::new(file);

        let empty = vec![String::from("")];
        let mut rows = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                let record = line.split("\t").map(&str::to_string).collect::<Vec<_>>();
                if record != empty {
                    rows.push(record);
                }
            } else if let Err(error) = line {
                todo!("{}", error)
            }
        }

        Self::from(rows)
    }
}

impl From<Vec<Vec<&str>>> for Sheet {
    fn from(rows_references: Vec<Vec<&str>>) -> Self {
        let rows = rows_references.iter()
            .map(|row| row.iter().map(|&value| value.to_string()).collect())
            .collect();
        Self { rows }
    }
}

impl From<Vec<Vec<String>>> for Sheet {
    fn from(rows: Vec<Vec<String>>) -> Self {
        Self { rows }
    }
}

impl Sheet {
    pub fn rows(self) -> IntoIter<Vec<String>> {
        self.rows.into_iter()
    }

    pub fn create_year_field(mut self, date_field_name: &str, year_field_name: &str) -> Self {
        let mut rows = self.rows.iter_mut();
        let schema_vec = rows.next().unwrap();
        schema_vec.push(year_field_name.to_string());
        let date_field_index = schema_vec.iter().position(|name| name == date_field_name).unwrap();
        let mut copy_year = String::new();
        for row in rows {
            let date = &row[date_field_index];
            if date.is_empty() {
                row.push(copy_year.clone())
            } else {
                let (year, _) = date.split_at(4);
                copy_year = year.to_string();
                row.push(year.to_string())
            }
        }
        self
    }
}

pub struct Schema {
    names: Vec<String>,
}

impl From<Vec<String>> for Schema {
    fn from(names: Vec<String>) -> Self {
        Self { names }
    }
}

impl Schema {
    pub fn selector<const N: usize>(&self, take_names: [&str; N]) -> impl Fn(Vec<String>) -> VecDeque<String> {
        let mut indexes = vec![];
        for take_name in take_names {
            let Some(index) = self.names.iter().position(|name| name == take_name) else { todo!("no name {take_name} in schema {:?}", self.names) };
            indexes.push(index);
        }

        let mut indexes_in_reverse_order: [usize; N] = indexes.clone().try_into().unwrap();
        indexes_in_reverse_order.sort();
        indexes_in_reverse_order.reverse();

        move |mut record| {
            let mut map = BTreeMap::new();
            for index in &indexes_in_reverse_order {
                map.insert(index, record.remove(*index));
            }

            let mut result: VecDeque<String> = VecDeque::new();
            for index in indexes.iter() {
                result.push_back(map.remove(index).unwrap());
            }
            result
        }
    }
}
