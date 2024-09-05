use std::io;
use std::path::Path;

fn main() {
    println!("file name: ");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)
    .expect("failed");

    let file_name = file_name.trim();
    let path = Path::new(file_name);

    if path.exists() {
        println!("exist {}", file_name);
    } else {
        println!("does not exist {}", file_name);
    }
}
