mod csv_random_picker;
mod translate_shell_client;

use std::env;
use std::io;

use csv_random_picker::CsvRandomPicker;
use translate_shell_client::TranslateShellClient;

/// Execute question repeatedly.
///
/// # Examples
///
/// ```
/// use std::env;
/// fn main() {
///     // expected args is ["program_name", "/tmp/input_file_name.tsv"]
///     eow_word_quiz::run(env::args());
/// }
/// ```
pub fn run(mut args: env::Args) {
    // 最初はファイル名なので流す
    args.next();

    let file_path = String::from(args.next().unwrap());
    let picker = CsvRandomPicker::new(&file_path).unwrap();

    loop {
        let some_row = picker.random_pickup().unwrap();
        println!(
        "Do you remember what '{}' means that you searched at {}?\nPress enter if you want answers.",
        some_row.word, some_row.date);

        let mut some_input = String::new();
        io::stdin().read_line(&mut some_input).ok(); // Errでもなんでも構わない

        // TODO(fugashy)
        // ここで毎回生成する必要はないのだが，Subprocessの使い方がイケてないのかエラーになってしまう
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
}
