mod csv_random_picker;
mod translate_shell_client;

use std::io;

use csv_random_picker::CsvRandomPicker;
use translate_shell_client::TranslateShellClient;

pub fn run() {
    let file_path = String::from("/tmp/words.tsv");
    let picker = CsvRandomPicker::new(&file_path).unwrap();

    let some_row = picker.random_pickup().unwrap();
    println!(
        "Do you remember what '{}' means that you searched at {}?\nPress enter if you want answers.",
        some_row.word, some_row.date
    );

    let mut some_input = String::new();
    io::stdin().read_line(&mut some_input).ok(); // Errでもなんでも構わない

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
