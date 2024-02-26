use serde::Deserialize;

use crate::{common::negative_default_i32, error::CLIError};

// TODO: find out what version I have been working with, create an enum for the supported versions
// that we can source from for converting to JSON

// TODO: define a trait which each version of CBL implements to convert to valid JSON?

// A CBL file as a Rust struct, derived from the XML schema above
#[derive(Debug, Deserialize)]
pub struct CBL {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "NumIssues")]
    pub num_issues: i32,
    #[serde(alias = "Books")]
    pub items: ComicReadingListItems,
}

impl CBL {
    pub fn from_file(file: &std::path::Path) -> Result<Self, CLIError> {
        let file = std::fs::File::open(file).map_err(|_| CLIError::FileNotFound)?;
        serde_xml_rs::from_reader(file).map_err(|e| CLIError::InvalidCBL(e.to_string()))
    }
}

#[derive(Debug, Deserialize)]
pub struct ComicReadingListItems {
    #[serde(alias = "Book", rename = "$value")]
    pub books: Vec<ComicReadingListItemBook>,
}

#[derive(Debug, Deserialize)]
pub struct ComicReadingListItemBook {
    #[serde(alias = "Id", default)]
    pub id: String,
    #[serde(alias = "FileName", default)]
    pub file_name: String,
    #[serde(alias = "Series", default)]
    pub series: String,
    #[serde(alias = "Number", default)]
    pub number: String,
    #[serde(alias = "Volume", default = "negative_default_i32")]
    pub volume: i32,
    #[serde(alias = "Year", default = "negative_default_i32")]
    pub year: i32,
    #[serde(alias = "Format", default)]
    pub format: String,
    #[serde(alias = "Database", default)]
    pub database: Option<ComicReadingListItemBookDatabase>,
}

#[derive(Debug, Deserialize)]
pub struct ComicReadingListItemBookDatabase {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Series")]
    pub series: String,
    #[serde(alias = "Issue")]
    pub issue: String,
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
