extern crate eow_words_quiz;
use eow_words_quiz::csv_random_picker::CsvRandomPicker;

fn main() {
    let file_path = String::from("/tmp/words.tsv");
    let mut picker = CsvRandomPicker::new(&file_path).unwrap();
    let some_row = picker.random_pickup().unwrap();
    println!("{}: {}", some_row.word, some_row.date);
}
