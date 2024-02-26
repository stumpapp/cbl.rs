use clap::ValueEnum;

use crate::cbl::CBL;

use self::draft::spec::DraftSpec;

mod draft;

pub const CURRENT_SPEC_VERSION: &str = draft::spec::VERSION;

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum SpecVersion {
    #[default]
    #[clap(name = "1.0-draft")]
    Draft,
}

impl ToString for SpecVersion {
    fn to_string(&self) -> String {
        match self {
            SpecVersion::Draft => draft::spec::VERSION.to_string(),
        }
    }
}

pub fn convert(cbl: CBL, version: SpecVersion) -> String {
    match version {
        SpecVersion::Draft => {
            let draft = DraftSpec::from(cbl);
            serde_json::to_string(&draft).expect("Failed to convert to JSON")
        }
    }
}
