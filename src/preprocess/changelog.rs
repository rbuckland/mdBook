use regex::Regex;
use std::path::Path;

use serde_json::{ Value, from_str };
use semver::Version;
use jsonpath::Selector;

use errors::*;

use super::{Preprocessor, PreprocessorContext};
use book::{Book, BookItem};

/// A preprocessor that generates a CHANGELOG.md file for the published book
pub struct ChangelogPreprocessor;

impl ChangelogPreprocessor {
    /// Create a new `ChangelogPreprocessor`.
    pub fn new() -> Self {
        ChangelogPreprocessor
    }
}

impl Preprocessor for ChangelogPreprocessor {
    fn name(&self) -> &str {
        "changelog"
    }

    fn run(&self, ctx: &PreprocessorContext, book: &mut Book) -> Result<()> {
        Ok(())
    }
}

enum FileType {
    JSON,
    YAML,
    TOML,
    RAW
}

/// Gneerate a CHANGELOG.md contents from git
struct GitSCMChangelogGenerator;

trait SCMChangelogGenerator {
    fn generate_changelog(file_path: &str, filetype: FileType, version_path: Option<&str>);
}

impl SCMChangelogGenerator for GitSCMChangelogGenerator {
   fn generate_changelog(file_path: &str, filetype: FileType, version_path: Option<&str>) {

   }
}

/// Flexible way to locate a SEMVER version from a file type

trait SemverLocator {
    fn get_semver(&self, contents: &str) -> Version;
}

struct JSONSemverLocator {
    json_path_selector: String
}

impl JSONSemverLocator {
    fn new(json_path_selector: &str) -> Self {
        JSONSemverLocator { json_path_selector: String::from(json_path_selector) }
    }
}

impl SemverLocator for JSONSemverLocator {
    fn get_semver(&self, contents: &str) -> Version {
        let json: Value = from_str(contents).unwrap();
        let selector = Selector::new(self.json_path_selector.as_str()).unwrap();
        let semver: &str = selector.find(&json)
        .map(|t| t.as_str().unwrap())
        .collect::<Vec<&str>>()[0];
        Version::parse(semver).unwrap()
    }
}

#[test]

fn test_can_read_json_semver() {
    let jsondoc = r#"
        {
            "releases": [
                 {
                     "version": "0.2.1"
                 },
                 {
                     "version": "0.2.2"
                 }
            ]
        }
    "#;

    let js = JSONSemverLocator::new("$.releases[0].version");

    assert_eq!(js.get_semver(jsondoc), Version::parse("0.2.1").unwrap());
}
