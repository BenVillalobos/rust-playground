use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    read_file("msbuild.log");
    
    Ok(())
}

fn read_file(file_name: &str) {
    let file = File::open(file_name).expect("File not found!");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let s = &line.unwrap().to_string();
        if s.contains("Environment") {
            println!("Environment started!");
            break;
        }
    }
}