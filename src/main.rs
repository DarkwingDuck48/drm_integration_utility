use std::{env, path};
use std::fs;

use std::io::Read;

fn read_file(file: &fs::File) -> String{
    let mut rdr = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::UTF_16LE))
            .build(file);
    let mut content = String::new();
    rdr.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = path::Path::new(&args[1]);
    if !file_path.exists() {
        panic!("File not exist")
    }
    let file = fs::File::open(file_path).unwrap();
    let file_content = read_file(&file);
    let all_lines = file_content.lines();
    let mut i = 0;
    for line in all_lines {
        println!("{}", line);
        i += 1;
        if i > 20 {
            break;
        }
    }
}
