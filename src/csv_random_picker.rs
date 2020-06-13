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

pub struct CsvRandomPicker {
    rows: Vec<Row>,
}

impl CsvRandomPicker {
    pub fn new(file_path: &str) -> Result<CsvRandomPicker, Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .comment(Some(b'#'))
            .delimiter(b'\t')
            .from_path(file_path)?;
        let mut rows: Vec<Row> = Vec::new();
        for record in reader.records() {
            rows.push(Row::new(record?)?);
        }
        Ok(CsvRandomPicker { rows })
    }

    // ランダムで行をとってくる
    pub fn random_pickup(&mut self) -> Result<&Row, Box<dyn Error>> {
        let pickup_id = rand::thread_rng().gen_range(0, self.rows.len());
        for row in self.rows.iter().enumerate() {
            if row.0 == pickup_id {
                return Ok(row.1);
            }
        }
        Err(From::from("Failed to pick up word"))
    }
}
