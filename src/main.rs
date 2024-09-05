mod file_reader;
mod user_interface;

use std::io;
use file_reader::count_words_in_file;
use user_interface::{ask_for_filename, display_results};

fn main() -> io::Result<()> {
    let filename = ask_for_filename()?;

    match count_words_in_file(&filename) {
        Ok(word_count) => display_results(&word_count),
        Err(_e) => eprintln!("Error!!!"),
    }

    Ok(())
}
