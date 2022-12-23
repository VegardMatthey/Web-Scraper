extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let data_path = r"/Users/cb/repo/web/data/";
    let creds_path = r"/Users/cb/repo/web/login.txt";
    let overview = file_to_text(data_path.to_owned() + "overview");
    let last = file_to_text(String::from(creds_path))
        .split("\n")
        .last()
        .expect("failed to read login.txt")
        .to_string();
    write_id(overview, data_path, creds_path, last).expect("failed");
    Ok(())
}

fn file_to_text(file_path: String) -> String {
    let file = File::open(file_path).expect("failed to open");
    let mut reader = BufReader::new(file);
    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();
    text
}

fn write_id(
    overview: String,
    data_path: &str,
    creds_path: &str,
    last: String,
) -> std::io::Result<u16> {
    let id = Regex::new(r#"(?m)id="email_broadcast_(\d*)""#).unwrap();
    let caps = id.captures_iter(&overview);
    let mut current;
    let mut emails = 0;
    let mut i: u16 = 0;
    let mut result = String::new();
    for mat in caps {
        current = String::from(mat.get(1).map_or("", |m| m.as_str())).to_string();
        if current == last {
            emails = i;
        }
        result += &(current + "\n")[..];
        i += 1;
    }

    let mut file = File::create(data_path.to_owned() + "links.txt").expect("failed to create file");
    file.write(result[..result.len() - 1].as_bytes())
        .expect("failed to write result");

    write_last(creds_path, data_path, emails);

    Ok(emails)
}

fn write_last(creds_path: &str, data_path: &str, emails: u16) -> String {
    let mut string: String = String::new();
    let mut i = 0;
    let text = file_to_text(String::from(creds_path));
    let size = text.split("\n").count();
    for item in text.split("\n") {
        string += item;
        if i > size - 4 {
            let mut file = File::create(creds_path).expect("failed to create login.txt");
            file.write((string.to_string() + "\n").as_bytes())
                .expect("failed to write ststring");
            file.write((emails.to_string() + "\n").as_bytes())
                .expect("failed to write amount of emails");
            let text: String = file_to_text(data_path.to_owned() + "links.txt");
            let lines: Vec<&str> = text.split("\n").collect();
            let last_id = lines[0].to_string();
            file.write(last_id.as_bytes())
                .expect("failed to write the last id");
            return string;
        } else {
            string += "\n";
        }
        i += 1;
    }
    string
}
