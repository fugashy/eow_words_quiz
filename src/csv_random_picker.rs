extern crate csv;
extern crate rand;

use rand::Rng;
use std::error::Error;

pub struct Row {
    pub word: String,
    pub date: String,
}
impl Row {
    pub fn new(input: csv::StringRecord) -> Result<Row, Box<dyn Error>> {
        if input.len() != 4 {
            return Err(From::from("Length of input must be 4"));
        }
        Ok(Row {
            word: input[0].to_string(),
            date: input[2].to_string(),
        })
    }
}

pub struct CsvRandomPicker<'a> {
    // private
    rows: usize,
    file_path: &'a str,
}

impl CsvRandomPicker<'_> {
    pub fn new<'a>(file_path: &'a str) -> Result<CsvRandomPicker, Box<dyn Error>> {
        let rows = csv::ReaderBuilder::new()
            .has_headers(false)
            .comment(Some(b'#'))
            .delimiter(b'\t')
            .from_path(file_path)?
            .records()
            .count();
        println!("Initialized with: '{}' that have {} lines", file_path, rows);
        Ok(CsvRandomPicker { rows, file_path })
    }

    // ランダムで行をとってくる
    pub fn random_pickup(&mut self) -> Result<Row, Box<dyn Error>> {
        let pickup_id = rand::thread_rng().gen_range(0, self.rows);
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .comment(Some(b'#'))
            .delimiter(b'\t')
            .from_path(self.file_path)?;
        for record in reader.records().enumerate() {
            if record.0 == pickup_id {
                return Row::new(record.1?);
            }
        }
        Err(From::from("Failed to pick up word"))
    }
}
