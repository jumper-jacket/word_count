use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn count_words_in_file(file_name: &str) -> io::Result<HashMap<String, u32>> {
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut word_count = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    Ok(word_count)
}
