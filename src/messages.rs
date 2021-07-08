use crate::csv_parser::parse_csv_file;
use crate::file_management::walk_dir_and_match_ext;
use std::fs::read;
use std::path::Path;

pub fn get_messages_from_dir(path: &Path) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();

    for entry in walk_dir_and_match_ext(path, "csv") {
        for item in parse_csv_file(&read(entry.path()).expect("failed to read file")[..]) {
            items.push(item.contents);
            if let Some(attachments) = item.attachments {
                items.push(attachments);
            }
        }
    }

    items
}
