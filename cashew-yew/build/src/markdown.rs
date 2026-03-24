use anyhow::Result;
use comrak::{markdown_to_html, Options};
use regex::Regex;

use crate::alerts;
use crate::magic_links;
use crate::slugify::slugify;

/// Render markdown content to HTML, applying the full plugin chain:
/// 1. Pre-process magic links and GitHub alerts
/// 2. Parse markdown to HTML via comrak (CommonMark + GFM)
/// 3. Add heading anchors with custom slugify
/// 4. Add target="_blank" to external links
pub fn render(content: &str) -> Result<String> {
    // Pre-process: expand magic links before markdown parsing
    let content = magic_links::process(content);

    // Pre-process: transform GitHub alerts
    let content = alerts::process(&content);

    // Parse markdown with comrak (CommonMark + GFM extensions)
    let mut options = Options::default();
    options.render.unsafe_ = true; // Allow raw HTML pass-through
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;

    let html = markdown_to_html(&content, &options);

    // Post-process: add heading anchors
    let html = add_heading_anchors(&html);

    // Post-process: add external link attributes
    let html = add_external_link_attrs(&html);

    Ok(html)
}

/// Add anchor links to headings with custom slugify.
fn add_heading_anchors(html: &str) -> String {
    let re = Regex::new(r"<(h[1-6])>(.*?)</h[1-6]>").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let tag = &caps[1];
        let text = &caps[2];
        // Strip HTML tags to get plain text for slugify
        let plain = Regex::new(r"<[^>]+>").unwrap().replace_all(text, "");
        let slug = slugify(&plain);
        format!(
            r##"<{tag} id="{slug}"><a class="header-anchor" href="#{slug}" aria-hidden="true">#</a>{text}</{close}>"##,
            tag = tag,
            slug = slug,
            text = text,
            close = tag,
        )
    })
    .to_string()
}

/// Add target="_blank" and rel="noopener" to external links.
fn add_external_link_attrs(html: &str) -> String {
    let re = Regex::new(r#"<a\s+href="(https?://[^"]+)"([^>]*)>"#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let href = &caps[1];
        let rest = &caps[2];
        // Skip if already has target
        if rest.contains("target=") {
            return caps[0].to_string();
        }
        format!(r#"<a href="{href}" target="_blank" rel="noopener"{rest}>"#)
    })
    .to_string()
}
