use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{cbl::CBL, common::negative_default_i32};

pub const VERSION: &str = "1.0-draft";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DraftSpec {
    /// The details of the CBL file, including its version and unique identifier
    file_details: FileDetails,
    /// The details of the reading list, including its name, description, publisher, and
    /// other metadata
    list_details: ListDetails,
    /// The actual issues in the reading list
    issues: Vec<ListIssue>,
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
    associated: Option<ListAssociations>,
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
    ///
    /// Note: this is a string because some issue numbers are alphanumeric, e.g. "1.HU"
    issue_num: String,
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
    ///
    /// Note: this is a string because some identifiers are alphanumeric
    pub series_id: String,
    /// The unique identifier of the [ListIssue] in the database
    ///
    /// Note: this is a string because some identifiers are alphanumeric
    pub issue_id: String,
}

impl From<CBL> for DraftSpec {
    fn from(value: CBL) -> Self {
        let min_year = value
            .items
            .books
            .iter()
            .map(|b| b.year)
            .min()
            .unwrap_or(negative_default_i32());

        let max_year = value
            .items
            .books
            .iter()
            .map(|b| b.year)
            .max()
            .unwrap_or(negative_default_i32());

        DraftSpec {
            file_details: FileDetails {
                version: VERSION.to_string(),
                // TODO: I assume this is fine, there is no unique ID in the CBL so when we convert it we will generate one ONCE
                // future versions will be operating on converting between spec versions, not CBL, so won't be generating unique IDs
                unique_id: Uuid::new_v4().to_string(),
            },
            list_details: ListDetails {
                name: value.name,
                description: None,            // TODO: where to get this?
                publisher: String::default(), // TODO: where to get this?
                imprint: String::default(),   // TODO: where to get this?
                start_year: min_year,
                end_year: max_year,
                r#type: String::default(), // TODO: where to get this?
                // associated: ListAssociations { // TODO: where to get this?
                //     characters: value.associated_characters,
                //     teams: value.associated_teams,
                // },
                associated: None,         // TODO: where to get this?
                cover_image_urls: vec![], // TODO: where to get this?
                relationships: None,      // TODO: where to get this?
            },
            issues: value
                .items
                .books
                .into_iter()
                .map(|i| ListIssue {
                    series_name: i.series,
                    series_start_year: i.year, // TODO: Not sure this is correct
                    issue_num: i.number,
                    issue_type: String::default(), // TODO: where to get this?
                    notes: None,
                    cover_date: String::default(), // TODO: where to get this?
                    databases: i
                        .database
                        .map(|d| {
                            vec![Database {
                                name: d.name,
                                series_id: d.series,
                                issue_id: d.issue,
                            }]
                        })
                        .unwrap_or_default(),
                })
                .collect(),
        }
    }
}
