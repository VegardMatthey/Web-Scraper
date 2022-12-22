extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let data_path = r"/Users/cb/repo/web/data/";
    let overview = file_to_text(data_path.to_owned() + "overview");
    write_id(overview, data_path).expect("failed");
    Ok(())
}

fn file_to_text(file_path: String) -> String {
    let file = File::open(file_path).expect("failed to open");
    let mut reader = BufReader::new(file);
    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();
    text
}

fn write_id(overview: String, data_path: &str) -> std::io::Result<()> {
    let mut file = File::create(data_path.to_owned() + "links.txt").expect("failed to create file");
    let id = Regex::new(r#"(?m)id="email_broadcast_(\d*)""#).unwrap();

    let caps = id.captures_iter(&overview);
    let mut current;

    let mut i = 0;
    for mat in caps {
        current = String::from(mat.get(1).map_or("", |m| m.as_str())).to_string() + "\n";
        file.write(current.as_bytes()).expect("failed to write");
    }

    Ok(())
}
