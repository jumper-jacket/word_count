use std::io::{self, Write};
use std::collections::HashMap;

pub fn ask_for_filename() -> io::Result<String> {
    println!("file name:");
    io::stdout().flush()?;

    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    Ok(filename.trim().to_string())
}

pub fn display_results(word_count: &HashMap<String, u32>) {
    for (word, count) in word_count.iter() {
        println!("{}: {}",word, count);
    }
}
