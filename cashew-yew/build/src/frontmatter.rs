use anyhow::Result;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};

/// All frontmatter fields supported by cashew.me markdown files.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frontmatter {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub display: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub place: Option<String>,
    #[serde(rename = "placeLink", default)]
    pub place_link: Option<String>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub draft: Option<bool>,
    #[serde(default)]
    pub art: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub redirect: Option<String>,
    #[serde(rename = "wrapperClass", default)]
    pub wrapper_class: Option<String>,
    #[serde(rename = "tocAlwaysOn", default)]
    pub toc_always_on: Option<bool>,
    #[serde(default)]
    pub class: Option<String>,

    // Structured data fields (experience, projects, etc.)
    // These are passed through as raw YAML values
    #[serde(default)]
    pub experience: Option<serde_yaml::Value>,
    #[serde(default)]
    pub projects: Option<serde_yaml::Value>,
}

/// Parsed page data: frontmatter + markdown content body.
pub struct PageData {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub is_full_width: bool,
}

/// Parse a markdown file's frontmatter and body content.
pub fn parse(raw: &str) -> Result<PageData> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(raw);

    let frontmatter: Frontmatter = result
        .data
        .map(|d| d.deserialize())
        .transpose()?
        .unwrap_or_default();

    let content = result.content;

    // Check for @layout-full-width comment
    let is_full_width = content.contains("@layout-full-width");

    Ok(PageData {
        frontmatter,
        content,
        is_full_width,
    })
}
