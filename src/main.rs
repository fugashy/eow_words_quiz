extern crate eow_words_quiz;
use eow_words_quiz::{
    csv_random_picker::CsvRandomPicker, translate_shell_client::TranslateShellClient,
};

fn main() {
    let file_path = String::from("/tmp/words.tsv");
    let picker = CsvRandomPicker::new(&file_path).unwrap();
    let some_row = picker.random_pickup().unwrap();
    println!(
        "Do you remember what '{}' means that you searched at {}",
        some_row.word, some_row.date
    );

    let mut translator = TranslateShellClient::new().unwrap();
    match translator.trans(&some_row.word) {
        Ok(v) => println!(
            "{}\n{}",
            v.0.unwrap_or("".to_string()),
            v.1.unwrap_or("".to_string())
        ),
        Err(e) => println!("{}", e),
    }
}
