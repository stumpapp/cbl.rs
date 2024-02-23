use clap::ValueEnum;

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
