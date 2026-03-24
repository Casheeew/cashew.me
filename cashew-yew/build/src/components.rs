use anyhow::Result;
use regex::Regex;
use serde_json::json;
use tera::{Context, Tera};

use crate::frontmatter::Frontmatter;

/// Photo metadata for the PhotoGalleryAll component.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PhotoMeta {
    pub name: String,
    pub url: String,
}

/// Demo metadata for the ListDemos component.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DemoMeta {
    pub date: String,
    pub date_display: String,
    pub link: Option<String>,
    pub description: String,
    pub video_url: String,
}

/// Post metadata collected during route discovery, used for ListPosts rendering.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PostMeta {
    pub path: String,
    pub title: String,
    pub date: String,
    pub date_display: String,
    pub duration: Option<String>,
    pub lang: Option<String>,
    pub redirect: Option<String>,
    #[serde(rename = "_year")]
    pub year: String,
}

/// Replace Vue component tags in rendered HTML with Tera-rendered partials.
/// Returns the HTML with components expanded.
pub fn expand_vue_components(
    html: &str,
    frontmatter: &Frontmatter,
    tera: &Tera,
    posts: &[PostMeta],
    photos: &[PhotoMeta],
    demos: &[DemoMeta],
) -> Result<String> {
    let mut result = html.to_string();

    // Replace <ListExperience ... />
    if let Some(experience) = &frontmatter.experience {
        let re = Regex::new(r#"<ListExperience[^>]*/>"#).unwrap();
        if re.is_match(&result) {
            let rendered = render_experience(experience, tera)?;
            result = re.replace(&result, rendered.as_str()).to_string();
        }
        // Also handle the case where comrak wraps it in <p> tags
        let re_p = Regex::new(r#"<p><ListExperience[^>]*/></p>"#).unwrap();
        if re_p.is_match(&result) {
            let rendered = render_experience(experience, tera)?;
            result = re_p.replace(&result, rendered.as_str()).to_string();
        }
    }

    // Replace <ListProjects ... />
    if let Some(projects) = &frontmatter.projects {
        let re = Regex::new(r#"<ListProjects[^>]*/>"#).unwrap();
        if re.is_match(&result) {
            let rendered = render_projects(projects, tera)?;
            result = re.replace(&result, rendered.as_str()).to_string();
        }
        let re_p = Regex::new(r#"<p><ListProjects[^>]*/></p>"#).unwrap();
        if re_p.is_match(&result) {
            let rendered = render_projects(projects, tera)?;
            result = re_p.replace(&result, rendered.as_str()).to_string();
        }
    }

    // Replace <ListPosts ... /> and <SubNav />
    let re_posts = Regex::new(r#"<ListPosts[^>]*/>"#).unwrap();
    let re_subnav = Regex::new(r#"<SubNav\s*/>"#).unwrap();
    if re_posts.is_match(&result) || re_subnav.is_match(&result) {
        let rendered = render_posts(posts, tera)?;
        // SubNav is part of the list_posts template, remove it separately
        result = re_subnav.replace(&result, "").to_string();
        let re_p_subnav = Regex::new(r#"<p><SubNav\s*/></p>"#).unwrap();
        result = re_p_subnav.replace(&result, "").to_string();
        result = re_posts.replace(&result, rendered.as_str()).to_string();
        let re_p_posts = Regex::new(r#"<p><ListPosts[^>]*/></p>"#).unwrap();
        result = re_p_posts.replace(&result, rendered.as_str()).to_string();
    }

    // Replace <PhotoGalleryAll ... />
    let re_photos = Regex::new(r#"<PhotoGalleryAll[^>]*/>"#).unwrap();
    if re_photos.is_match(&result) {
        let rendered = render_photos(photos, tera)?;
        result = re_photos.replace(&result, rendered.as_str()).to_string();
    }
    let re_photos_p = Regex::new(r#"<p><PhotoGalleryAll[^>]*/></p>"#).unwrap();
    if re_photos_p.is_match(&result) {
        let rendered = render_photos(photos, tera)?;
        result = re_photos_p.replace(&result, rendered.as_str()).to_string();
    }

    // Replace <ListDemos />
    let re_demos = Regex::new(r#"<ListDemos\s*/>"#).unwrap();
    if re_demos.is_match(&result) {
        let rendered = render_demos(demos, tera)?;
        result = re_demos.replace(&result, rendered.as_str()).to_string();
    }
    let re_demos_p = Regex::new(r#"<p><ListDemos\s*/></p>"#).unwrap();
    if re_demos_p.is_match(&result) {
        let rendered = render_demos(demos, tera)?;
        result = re_demos_p.replace(&result, rendered.as_str()).to_string();
    }

    // Replace <CustomBackground /> with the actual background div
    let custom_bg_html = r#"<div id="custom-bg" style="position:fixed;top:-1vh;left:0;bottom:0;width:100vw;height:101vh;background-position:center center;background-repeat:no-repeat;background-size:cover;background-attachment:fixed;filter:blur(0px);background-blend-mode:overlay;z-index:-1;"></div>"#;
    let re_custom_bg = Regex::new(r#"<CustomBackground\s*/>"#).unwrap();
    result = re_custom_bg.replace_all(&result, custom_bg_html).to_string();
    let re_custom_bg_p = Regex::new(r#"<p><CustomBackground\s*/></p>"#).unwrap();
    result = re_custom_bg_p.replace_all(&result, custom_bg_html).to_string();

    // Remove other unrecognized Vue components (they'd render as empty anyway)
    let re_vue = Regex::new(r#"<(MediaConsumption|SponsorButtons)\s*/>"#).unwrap();
    result = re_vue.replace_all(&result, "").to_string();
    let re_vue_p = Regex::new(r#"<p><(MediaConsumption|SponsorButtons)\s*/></p>"#).unwrap();
    result = re_vue_p.replace_all(&result, "").to_string();

    Ok(result)
}

/// Render the ListExperience component.
fn render_experience(experience_yaml: &serde_yaml::Value, tera: &Tera) -> Result<String> {
    // Convert YAML to JSON for Tera
    let experience_json: serde_json::Value = serde_yaml::from_value(experience_yaml.clone())?;

    // Add _year field to each experience item
    let mut items = match experience_json {
        serde_json::Value::Array(arr) => arr,
        _ => vec![],
    };

    for item in &mut items {
        if let Some(duration) = item.get("duration").and_then(|d| d.as_str()) {
            let year = extract_end_year(duration);
            item.as_object_mut()
                .unwrap()
                .insert("_year".to_string(), json!(year));
        }
    }

    // Sort by year descending
    items.sort_by(|a, b| {
        let ya = a.get("_year").and_then(|y| y.as_str()).unwrap_or("");
        let yb = b.get("_year").and_then(|y| y.as_str()).unwrap_or("");
        yb.cmp(ya)
    });

    let mut ctx = Context::new();
    ctx.insert("experience", &items);

    let rendered = tera.render("partials/list_experience.html", &ctx)?;
    Ok(rendered)
}

/// Render the ListProjects component.
fn render_projects(projects_yaml: &serde_yaml::Value, tera: &Tera) -> Result<String> {
    // Convert YAML to JSON for Tera
    let projects_json: serde_json::Value = serde_yaml::from_value(projects_yaml.clone())?;

    let mut ctx = Context::new();
    ctx.insert("projects", &projects_json);

    let rendered = tera.render("partials/list_projects.html", &ctx)?;
    Ok(rendered)
}

/// Render the ListPosts component.
fn render_posts(posts: &[PostMeta], tera: &Tera) -> Result<String> {
    let mut ctx = Context::new();
    ctx.insert("posts", posts);

    let rendered = tera.render("partials/list_posts.html", &ctx)?;
    Ok(rendered)
}

/// Extract the end year from a duration string like "Present - 02/2027" or "12/2023 - 02/2024".
/// Returns the first year found (which represents the most recent/end date).
fn extract_end_year(duration: &str) -> String {
    if duration.contains("Present") {
        return "2025".to_string();
    }

    // Find all 4-digit year patterns
    let re = Regex::new(r"\b(\d{4})\b").unwrap();
    let years: Vec<&str> = re.find_iter(duration).map(|m| m.as_str()).collect();

    // Return the last year (which is typically the end date)
    years.last().unwrap_or(&"2025").to_string()
}

/// Collect post metadata from all routes for the ListPosts component.
pub fn collect_post_metadata(
    pages_dir: &std::path::Path,
) -> Result<Vec<PostMeta>> {
    use gray_matter::engine::YAML;
    use gray_matter::Matter;
    use std::fs;
    use walkdir::WalkDir;

    let posts_dir = pages_dir.join("posts");
    if !posts_dir.exists() {
        return Ok(vec![]);
    }

    let matter = Matter::<YAML>::new();
    let mut posts = Vec::new();

    for entry in WalkDir::new(&posts_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        // Skip index.md (that's the listing page itself)
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name == "index.md" {
            continue;
        }

        let raw = fs::read_to_string(path)?;
        let result = matter.parse(&raw);

        let fm: crate::frontmatter::Frontmatter = result
            .data
            .map(|d| d.deserialize())
            .transpose()?
            .unwrap_or_default();

        // Skip drafts
        if fm.draft == Some(true) {
            continue;
        }

        let title = fm.title.unwrap_or_default();
        let date = fm.date.clone().unwrap_or_default();
        let date_display = format_date(&date);
        let year = extract_year_from_date(&date);

        // Compute URL path from file
        let relative = path.strip_prefix(pages_dir)?;
        let url_path = format!(
            "/{}",
            relative
                .with_extension("")
                .to_string_lossy()
                .replace('\\', "/")
        );

        posts.push(PostMeta {
            path: url_path,
            title,
            date: date.clone(),
            date_display,
            duration: fm.duration.clone(),
            lang: fm.lang.clone(),
            redirect: fm.redirect.clone(),
            year,
        });
    }

    // Sort by date descending
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

/// Format a date string like "2025-03-14T16:00:00Z" to "Mar 14, 2025".
fn format_date(date_str: &str) -> String {
    // Try to parse ISO 8601 date
    let date_part = date_str.split('T').next().unwrap_or(date_str);
    let parts: Vec<&str> = date_part.split('-').collect();

    if parts.len() >= 3 {
        let month = match parts[1] {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {}, {}", month, day, parts[0])
    } else {
        date_str.to_string()
    }
}

/// Extract the year from a date string.
fn extract_year_from_date(date_str: &str) -> String {
    let date_part = date_str.split('T').next().unwrap_or(date_str);
    let parts: Vec<&str> = date_part.split('-').collect();
    parts.first().unwrap_or(&"2025").to_string()
}

/// Render the PhotoGalleryAll component.
fn render_photos(photos: &[PhotoMeta], tera: &Tera) -> Result<String> {
    let mut ctx = Context::new();
    ctx.insert("photos", photos);
    let rendered = tera.render("partials/list_photos.html", &ctx)?;
    Ok(rendered)
}

/// Render the ListDemos component.
fn render_demos(demos: &[DemoMeta], tera: &Tera) -> Result<String> {
    let mut ctx = Context::new();
    ctx.insert("demos", demos);
    let rendered = tera.render("partials/list_demos.html", &ctx)?;
    Ok(rendered)
}

/// Collect photo metadata from the photos directory.
pub fn collect_photos(cashew_dir: &std::path::Path) -> Result<Vec<PhotoMeta>> {
    use std::fs;
    let photos_dir = cashew_dir.join("photos");
    if !photos_dir.exists() {
        return Ok(vec![]);
    }

    let mut photos = Vec::new();
    for entry in fs::read_dir(&photos_dir)? {
        let entry = entry?;
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp") {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let url = format!("/photos/{name}");
        photos.push(PhotoMeta { name, url });
    }

    // Sort by filename descending (newest photos have higher IMG_ numbers)
    photos.sort_by(|a, b| b.name.cmp(&a.name));
    Ok(photos)
}

/// Collect demo metadata from the demo directory.
pub fn collect_demos(cashew_dir: &std::path::Path) -> Result<Vec<DemoMeta>> {
    use gray_matter::engine::YAML;
    use gray_matter::Matter;
    use std::fs;
    use walkdir::WalkDir;

    let demo_dir = cashew_dir.join("demo");
    if !demo_dir.exists() {
        return Ok(vec![]);
    }

    let matter = Matter::<YAML>::new();
    let mut demos = Vec::new();

    for entry in WalkDir::new(&demo_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        // Expect filename like "2023-06-23"
        if stem.len() != 10 || !stem.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            continue;
        }

        // Check matching video exists
        let video_path = demo_dir.join(format!("{stem}.mp4"));
        if !video_path.exists() {
            continue;
        }

        let raw = fs::read_to_string(path)?;
        let parsed = matter.parse(&raw);

        // Extract link from frontmatter
        let link = parsed.data.as_ref().and_then(|d| {
            d.as_hashmap().ok()?.get("link")?.as_str().ok().map(|s| s.to_string())
        });

        // The body is the description
        let description = parsed.content.trim().to_string();

        let date_display = format_date_from_stem(&stem);

        demos.push(DemoMeta {
            date: stem.clone(),
            date_display,
            link,
            description,
            video_url: format!("/demo/{stem}.mp4"),
        });
    }

    // Sort by date descending
    demos.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(demos)
}

/// Format a date stem like "2023-06-23" to "Jun 23, 2023".
fn format_date_from_stem(stem: &str) -> String {
    let parts: Vec<&str> = stem.split('-').collect();
    if parts.len() == 3 {
        let month = match parts[1] {
            "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
            "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
            "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
            _ => parts[1],
        };
        let day = parts[2].trim_start_matches('0');
        format!("{} {}, {}", month, day, parts[0])
    } else {
        stem.to_string()
    }
}
