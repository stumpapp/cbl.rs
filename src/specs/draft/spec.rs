use serde::{Deserialize, Serialize};

use crate::common::negative_default_i32;

pub const VERSION: &str = "1.0-draft";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DraftSpec {
    file_details: FileDetails,
    list_details: ListDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileDetails {
    version: String,
    unique_id: String,
}

// TODO: determine what are actual optionals

/// The details of a reading list, including its name, description, publisher, and
/// other metadata
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDetails {
    /// The name of the reading list
    name: String,
    /// The description of the reading list, if any
    description: Option<String>,
    /// The publisher(s) of the reading list, concatenated with a comma as a
    // single, delimited string
    publisher: String,
    /// TODO: determine what this is
    imprint: String,
    /// The year in which the reading list chronologically begins. Should be
    /// determined by the start year of the first issue in the list
    #[serde(default = "negative_default_i32")]
    start_year: i32,
    /// The year in which the reading list chronologically ends. Should be
    /// determined by the end year of the last issue in the list
    #[serde(default = "negative_default_i32")]
    end_year: i32,
    /// The type of reading list, e.g. "Chronological", "Event", "Crossover". This
    /// can be a comma-delimited string if the list is of multiple types
    #[serde(alias = "Type")]
    r#type: String,
    /// Associated information about the reading list, e.g. characters, teams
    #[serde(default)]
    associated: ListAssociations,
    /// URLs for valid cover images for the reading list
    #[serde(rename = "CoverImageURLs", default)]
    cover_image_urls: Vec<String>,
    /// Relationship definitions for related reading lists, e.g. "Previous" and "Next",
    /// which indicates an order or sequence of reading lists
    relationships: Option<ListRelationships>,
}

// TODO: I think this would more aptly be named "ListSubjects"
/// The characters and teams associated with a reading list
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAssociations {
    characters: Vec<String>,
    teams: Vec<String>,
}

/// The relationships of a reading list to other reading lists, e.g. "Previous" and "Next",
/// if any exist. Relationships indicate an order or sequence of reading lists
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListRelationships {
    /// The previous reading list in the sequence, if any
    previous: Option<ListRelationship>,
    /// The next reading list in the sequence, if any
    next: Option<ListRelationship>,
}

/// The details of a related reading list, including its name and unique identifier
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListRelationship {
    /// The name of the related reading list. See [ListDetails::name]
    name: String,
    /// The unique identifier of the related reading list. See [FileDetails::unique_id]
    #[serde(rename = "ID")]
    id: String,
}

/// An issue in the reading list, representing a single comic book or graphic novel
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListIssue {
    /// The name of the series which the issue belongs to
    series_name: String,
    /// The year in which the series began
    #[serde(default = "negative_default_i32")]
    series_start_year: i32,
    /// The sequence number of the issue in the series
    issue_num: i32,
    /// The type of issue, e.g. "Event", "Event Tie-In", "Ongoing Series"
    issue_type: String,
    /// Additional information about the issue and/or its inclusion in the reading list
    notes: Option<String>,
    // TODO: what date format will draft accept?
    /// A date string representing the cover date of the issue, e.g. "2021-01-07"
    cover_date: String,
    /// A list of databases which the issue can be found in
    #[serde(default)]
    databases: Vec<Database>,
}

/// A database which a [ListIssue] can be found in/queryable from
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Database {
    /// The name of the database, e.g. "Comicvine" or "Grand Comics Database"
    pub name: String,
    /// The unique identifier of the series in the database which the [ListIssue]
    /// belongs to
    pub series_id: i32,
    /// The unique identifier of the [ListIssue] in the database
    pub issue_id: i32,
}
