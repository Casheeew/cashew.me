use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

/// A discovered route from a markdown file.
#[derive(Debug)]
pub struct Route {
    /// Absolute path to the source .md file
    pub source_path: PathBuf,
    /// URL path (e.g., "/posts/my-post")
    pub url_path: String,
    /// Whether this is a demo page (in /demo/ directory)
    pub is_demo: bool,
}

impl Route {
    /// Compute the output HTML file path within the dist directory.
    pub fn output_path(&self, dist_dir: &Path) -> PathBuf {
        if self.url_path == "/" {
            dist_dir.join("index.html")
        } else {
            let clean = self.url_path.trim_start_matches('/');
            dist_dir.join(clean).join("index.html")
        }
    }
}

/// Discover all routes from markdown files in the pages directory.
/// Maps file paths to URL paths following the same convention as unplugin-vue-router:
///   pages/index.md       -> /
///   pages/posts/foo.md   -> /posts/foo
///   pages/[...404].md    -> /404 (catch-all)
pub fn discover_routes(pages_dir: &Path) -> Result<Vec<Route>> {
    let mut routes = Vec::new();

    for entry in WalkDir::new(pages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Only process .md files
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let relative = path.strip_prefix(pages_dir)?;
        let url_path = file_path_to_url(relative);
        let is_demo = relative.starts_with("demo") || relative.starts_with("demos");

        routes.push(Route {
            source_path: path.to_path_buf(),
            url_path,
            is_demo,
        });
    }

    // Sort routes for deterministic output
    routes.sort_by(|a, b| a.url_path.cmp(&b.url_path));

    Ok(routes)
}

/// Convert a file path like `posts/my-post.md` to a URL path like `/posts/my-post`.
fn file_path_to_url(relative: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();

    for component in relative.components() {
        let s = component.as_os_str().to_string_lossy().to_string();
        parts.push(s);
    }

    // Remove .md extension from the last part
    if let Some(last) = parts.last_mut() {
        if let Some(stripped) = last.strip_suffix(".md") {
            *last = stripped.to_string();
        }
    }

    // Handle index files
    if parts.last().map(|s| s.as_str()) == Some("index") {
        parts.pop();
    }

    // Handle catch-all routes like [...404]
    for part in &mut parts {
        if part.starts_with("[...") && part.ends_with(']') {
            let inner = &part[4..part.len() - 1];
            *part = inner.to_string();
        }
    }

    if parts.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", parts.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_index() {
        assert_eq!(file_path_to_url(Path::new("index.md")), "/");
    }

    #[test]
    fn test_simple_page() {
        assert_eq!(file_path_to_url(Path::new("experience.md")), "/experience");
    }

    #[test]
    fn test_nested_page() {
        assert_eq!(
            file_path_to_url(Path::new("posts/my-post.md")),
            "/posts/my-post"
        );
    }

    #[test]
    fn test_catch_all() {
        assert_eq!(file_path_to_url(Path::new("[...404].md")), "/404");
    }

    #[test]
    fn test_nested_index() {
        assert_eq!(file_path_to_url(Path::new("posts/index.md")), "/posts");
    }
}
