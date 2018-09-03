/// Configuration for the HTML renderer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PdfConfig {
    pub generate: bool,
}
