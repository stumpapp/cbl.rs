use serde::Deserialize;

// TODO: find out what version I have been working with, create an enum for the supported versions
// that we can source from for converting to JSON

// TODO: define a trait which each version of CBL implements to convert to valid JSON?

// A CBL file as a Rust struct, derived from the XML schema above
#[derive(Debug, Deserialize)]
pub struct CBL {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "NumIssues")]
    num_issues: i32,
    #[serde(alias = "Books")]
    items: ComicReadingListItems,
}

fn negative_default() -> i32 {
    -1
}

#[derive(Debug, Deserialize)]
pub struct ComicReadingListItems {
    #[serde(alias = "Book", rename = "$value")]
    books: Vec<ComicReadingListItemBook>,
}

#[derive(Debug, Deserialize)]
pub struct ComicReadingListItemBook {
    #[serde(alias = "Id", default)]
    id: String,
    #[serde(alias = "FileName", default)]
    file_name: String,
    #[serde(alias = "Series", default)]
    series: String,
    #[serde(alias = "Number", default)]
    number: String,
    #[serde(alias = "Volume", default = "negative_default")]
    volume: i32,
    #[serde(alias = "Year", default = "negative_default")]
    year: i32,
    #[serde(alias = "Format", default)]
    format: String,
    #[serde(alias = "Database", default)]
    database: Option<ComicReadingListItemBookDatabase>,
}

#[derive(Debug, Deserialize)]
pub struct ComicReadingListItemBookDatabase {
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Series")]
    series: String,
    #[serde(alias = "Issue")]
    issue: String,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use super::*;

    #[test]
    fn load_basic_cbl() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/basic.cbl");
        let cbl = serde_xml_rs::from_reader::<_, CBL>(File::open(path).unwrap()).unwrap();

        assert_eq!(cbl.name, "[Spider-Man] 00 - Complete 616 Chronology");
        assert_eq!(cbl.items.books.len(), cbl.num_issues as usize);

        println!("{:?}", cbl);
    }
}