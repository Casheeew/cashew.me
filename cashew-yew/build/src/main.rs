mod alerts;
mod components;
mod frontmatter;
mod magic_links;
mod markdown;
mod router;
mod shiki;
mod slugify;

use std::fs;
use std::path::Path;

use anyhow::Result;
use tera::Tera;

use crate::router::Route;

fn main() -> Result<()> {
    let pages_dir = Path::new("../cashew.me/pages");
    let templates_dir = "templates/**/*";
    let dist_dir = Path::new("dist");

    // Initialize Tera templates
    let tera = Tera::new(templates_dir)?;

    // Discover routes from markdown files
    let routes = router::discover_routes(pages_dir)?;
    println!("Discovered {} routes", routes.len());

    let cashew_dir = Path::new("../cashew.me");

    // Collect post metadata for ListPosts component
    let posts = components::collect_post_metadata(pages_dir)?;
    println!("Collected {} blog posts", posts.len());

    // Collect photo metadata for PhotoGalleryAll component
    let photos = components::collect_photos(cashew_dir)?;
    println!("Collected {} photos", photos.len());

    // Collect demo metadata for ListDemos component
    let demos = components::collect_demos(cashew_dir)?;
    println!("Collected {} demos", demos.len());

    // Process each route
    for route in &routes {
        println!("Processing: {} -> {}", route.source_path.display(), route.url_path);
        match process_route(route, &tera, dist_dir, &posts, &photos, &demos) {
            Ok(()) => {}
            Err(e) => eprintln!("  Error: {e}"),
        }
    }

    // Copy static assets
    copy_static_assets(dist_dir)?;

    // Generate RSS feed
    generate_rss(&posts, dist_dir)?;
    println!("Generated RSS feed");

    println!("\nBuild complete! Output in {}", dist_dir.display());
    Ok(())
}

fn process_route(
    route: &Route,
    tera: &Tera,
    dist_dir: &Path,
    posts: &[components::PostMeta],
    photos: &[components::PhotoMeta],
    demos: &[components::DemoMeta],
) -> Result<()> {
    let raw = fs::read_to_string(&route.source_path)?;

    // Extract frontmatter
    let page_data = frontmatter::parse(&raw)?;

    // Handle redirects
    if let Some(ref redirect) = page_data.frontmatter.redirect {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head><meta http-equiv="refresh" content="0; url={redirect}"></head>
<body>Redirecting to <a href="{redirect}">{redirect}</a></body>
</html>"#
        );
        let out_path = route.output_path(dist_dir);
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out_path, html)?;
        return Ok(());
    }

    // Convert markdown to HTML
    let html_content = markdown::render(&page_data.content)?;

    // Highlight code blocks via Shiki
    let html_content = shiki::highlight_code_blocks(&html_content)?;

    // Expand Vue component tags into rendered HTML
    let html_content =
        components::expand_vue_components(&html_content, &page_data.frontmatter, tera, posts, photos, demos)?;

    // Determine template
    let template_name = if route.is_demo { "demo.html" } else { "post.html" };

    // Build template context
    let mut context = tera::Context::new();
    context.insert("content", &html_content);
    context.insert("frontmatter", &page_data.frontmatter);
    context.insert("route_path", &route.url_path);
    context.insert("is_full_width", &page_data.is_full_width);

    let parent_path = route
        .url_path
        .rsplit_once('/')
        .map(|(parent, _)| {
            if parent.is_empty() {
                "/".to_string()
            } else {
                parent.to_string()
            }
        })
        .unwrap_or_else(|| "/".to_string());
    context.insert("parent_path", &parent_path);

    // Social sharing URLs (pre-computed since Tera lacks urlencode)
    let base = "https://antfu.me";
    let page_url = format!("{base}{}", route.url_path);
    fn urlencode(s: &str) -> String {
        s.bytes()
            .map(|b| match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    (b as char).to_string()
                }
                _ => format!("%{b:02X}"),
            })
            .collect()
    }
    let bluesky_text = urlencode(&format!("Reading @antfu.me {page_url}\n\nI think..."));
    let mastodon_text = urlencode(&format!("Reading @antfu@m.webtoo.ls {page_url}\n\nI think..."));
    let twitter_text = urlencode(&format!("Reading @antfu7's {page_url}\n\nI think..."));
    context.insert("bluesky_url", &format!("https://bsky.app/intent/compose?text={bluesky_text}"));
    context.insert("mastodon_url", &format!("https://elk.zone/intent/post?text={mastodon_text}"));
    context.insert("twitter_url", &format!("https://twitter.com/intent/tweet?text={twitter_text}"));

    // Render template
    let rendered = tera.render(template_name, &context)?;

    // Write output
    let out_path = route.output_path(dist_dir);
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out_path, rendered)?;

    Ok(())
}

fn generate_rss(posts: &[components::PostMeta], dist_dir: &Path) -> Result<()> {
    let base = "https://casheeew.github.io/cashew.me";
    let now = chrono::Utc::now().to_rfc2822();

    let mut items = String::new();
    for post in posts {
        if post.lang.as_deref() == Some("zh") || post.lang.as_deref() == Some("ja") {
            continue;
        }
        let link = format!("{base}{}", post.path);
        let title = html_escape(&post.title);
        items.push_str(&format!(
            "    <item>\n      <title>{title}</title>\n      <link>{link}</link>\n      <guid>{link}</guid>\n      <pubDate>{}</pubDate>\n    </item>\n",
            &post.date
        ));
    }

    let rss = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>Cashew</title>
    <description>Cashew's Blog</description>
    <link>{base}/</link>
    <atom:link href="{base}/feed.xml" rel="self" type="application/rss+xml"/>
    <lastBuildDate>{now}</lastBuildDate>
{items}  </channel>
</rss>"#
    );

    fs::write(dist_dir.join("feed.xml"), &rss)?;

    // Also generate Atom feed
    let mut entries = String::new();
    for post in posts {
        if post.lang.as_deref() == Some("zh") || post.lang.as_deref() == Some("ja") {
            continue;
        }
        let link = format!("{base}{}", post.path);
        let title = html_escape(&post.title);
        entries.push_str(&format!(
            "  <entry>\n    <title>{title}</title>\n    <link href=\"{link}\"/>\n    <id>{link}</id>\n    <updated>{}</updated>\n  </entry>\n",
            &post.date
        ));
    }

    let atom = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Cashew</title>
  <subtitle>Cashew's Blog</subtitle>
  <link href="{base}/"/>
  <link href="{base}/feed.atom" rel="self"/>
  <updated>{now}</updated>
  <id>{base}/</id>
{entries}</feed>"#
    );

    fs::write(dist_dir.join("feed.atom"), &atom)?;
    Ok(())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn copy_static_assets(dist_dir: &Path) -> Result<()> {
    let static_dir = Path::new("static");
    if static_dir.exists() {
        copy_dir_recursive(static_dir, dist_dir)?;
    }

    // Copy styles to dist/styles/
    let styles_dir = Path::new("styles");
    if styles_dir.exists() {
        let dest = dist_dir.join("styles");
        fs::create_dir_all(&dest)?;
        copy_dir_recursive(styles_dir, &dest)?;
    }

    // Also copy public assets from cashew.me
    let public_dir = Path::new("../cashew.me/public");
    if public_dir.exists() {
        copy_dir_recursive(public_dir, dist_dir)?;
    }

    // Copy photos directory (images for the photos page)
    let photos_dir = Path::new("../cashew.me/photos");
    if photos_dir.exists() {
        let dest = dist_dir.join("photos");
        fs::create_dir_all(&dest)?;
        copy_dir_recursive(photos_dir, &dest)?;
    }

    // Copy demo videos (for the demos page)
    let demo_dir = Path::new("../cashew.me/demo");
    if demo_dir.exists() {
        let dest = dist_dir.join("demo");
        fs::create_dir_all(&dest)?;
        for entry in fs::read_dir(demo_dir)? {
            let entry = entry?;
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext == "mp4" {
                let name = path.file_name().unwrap();
                fs::copy(&path, dest.join(name))?;
            }
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(src)?;
        let target = dst.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
