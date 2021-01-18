use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

use colored::*;

fn main() -> std::io::Result<()> {
    // To Do: read current directory's msbuild.log unless user
    // specifies a filepath
    read_file("msbuild.log");
    Ok(())
}

fn read_file(file_name: &str) {
    let file = File::open(file_name).expect("File not found!");
    let reader = std::io::BufReader::new(file);
    let reg = Regex::new(r"MSB\d{4}").unwrap();


    let all_lines = reader.lines()
                          .map(|l| l.unwrap_or("".to_string()));
    
    for (line_number, line) in all_lines.enumerate() {
        if reg.is_match(&line) {
            // Each warning or error is broken up like so:
            // Timestamp? -
            // Some number and a '>'. PID? -
            // FilePath -
            // (Line, col):
            // warning <code>:
            // Actual message
            // [FilePath in brackets]

            let carat_index = line.find('>').unwrap_or(0);
            let end_of_file_path_index = line.find(": ").unwrap();
            let square_bracket_index = line.find('[').unwrap();

            
            let file_path = &line[carat_index+1..end_of_file_path_index];
            let actual_message = &line[end_of_file_path_index+1..square_bracket_index];
            let code = &actual_message[..actual_message.find(':').unwrap()];
            let project_file = &line[square_bracket_index+1..line.len()-1];

            println!("{1} {} {1}", code.yellow(), "----".yellow());
            println!("Line in Log: {}", line_number.to_string().blue());
            println!("File: {}", file_path.trim().green());
            println!("Project: {}", project_file.cyan());
            println!("Message: {}", actual_message.trim());
            println!();
            println!();

            //println!("Line {}: {}", x, line);
        }
    }
}