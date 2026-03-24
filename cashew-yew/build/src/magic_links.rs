use regex::Regex;
use std::collections::HashMap;

/// A magic link definition: display name maps to URL and optional image.
struct MagicLink {
    url: String,
    image_url: Option<String>,
}

/// Build the magic links map, ported from vite.config.ts lines 128-172.
fn build_links_map() -> HashMap<&'static str, MagicLink> {
    let mut map = HashMap::new();

    // Organizations / companies with custom images
    map.insert("Bering Lab", MagicLink {
        url: "https://beringlab.com".into(),
        image_url: Some("https://avatars.githubusercontent.com/u/88300039?v=4".into()),
    });
    map.insert("HCI Tech Lab", MagicLink {
        url: "https://hcitech.org".into(),
        image_url: Some("https://i.postimg.cc/0yRnbRfx/HCITech.png".into()),
    });
    map.insert("KAIST", MagicLink {
        url: "https://www.kaist.ac.kr/en/html/kaist/01.html".into(),
        image_url: Some("https://i.postimg.cc/tC3LD573/emblem2.gif".into()),
    });
    map.insert("AiGlow", MagicLink {
        url: "https://www.linkedin.com/company/aiglowedu/".into(),
        image_url: Some("https://media.licdn.com/dms/image/v2/D560BAQEg9ciHhdliBA/company-logo_200_200/B56ZfWmoXbHEAI-/0/1751652141428/aiglowedu_logo?e=1760572800&v=beta&t=0_3bb-_8eupMgjebvR0M7OVfKzSj7Gb2_BScfxByADk".into()),
    });
    map.insert("CHI 2026", MagicLink {
        url: "https://chi2026.acm.org/".into(),
        image_url: Some("https://chi2026.acm.org/wordpress/wp-content/uploads/2024/09/chi-banner-full.png".into()),
    });
    map.insert("Anthony Fu Collective", MagicLink {
        url: "https://opencollective.com/antfu".into(),
        image_url: Some("https://github.com/antfu-collective.png".into()),
    });
    map.insert("Netlify", MagicLink {
        url: "https://netlify.com".into(),
        image_url: Some("https://github.com/netlify.png".into()),
    });
    map.insert("Stackblitz", MagicLink {
        url: "https://stackblitz.com".into(),
        image_url: Some("https://github.com/stackblitz.png".into()),
    });
    map.insert("Vercel", MagicLink {
        url: "https://vercel.com".into(),
        image_url: Some("https://github.com/vercel.png".into()),
    });

    // GitHub repos (image auto-derived from org avatar)
    let github_repos: Vec<(&str, &str)> = vec![
        ("Yomitan", "https://github.com/yomidevs/yomitan"),
        ("Yomitan Wiki", "https://github.com/yomidevs/yomitan-wiki"),
        ("Slidev", "https://github.com/slidevjs/slidev"),
        ("VueUse", "https://github.com/vueuse/vueuse"),
        ("UnoCSS", "https://github.com/unocss/unocss"),
        ("Elk", "https://github.com/elk-zone/elk"),
        ("Type Challenges", "https://github.com/type-challenges/type-challenges"),
        ("Vue", "https://github.com/vuejs/core"),
        ("Nuxt", "https://github.com/nuxt/nuxt"),
        ("Vite", "https://github.com/vitejs/vite"),
        ("Shiki", "https://github.com/shikijs/shiki"),
        ("Twoslash", "https://github.com/twoslashes/twoslash"),
        ("TwoSlash", "https://github.com/twoslashes/twoslash"),
        ("ESLint Stylistic", "https://github.com/eslint-stylistic/eslint-stylistic"),
        ("Unplugin", "https://github.com/unplugin"),
        ("Nuxt DevTools", "https://github.com/nuxt/devtools"),
        ("Vite PWA", "https://github.com/vite-pwa"),
        ("i18n Ally", "https://github.com/lokalise/i18n-ally"),
        ("ESLint", "https://github.com/eslint/eslint"),
        ("Astro", "https://github.com/withastro/astro"),
    ];

    for (name, url) in github_repos {
        let image_url = github_avatar_url(url);
        map.insert(name, MagicLink {
            url: url.into(),
            image_url,
        });
    }

    // Image overrides for specific GitHub URLs
    apply_image_overrides(&mut map);

    map
}

/// Extract GitHub org avatar from a repo URL.
fn github_avatar_url(url: &str) -> Option<String> {
    if let Some(path) = url.strip_prefix("https://github.com/") {
        let org = path.split('/').next()?;
        Some(format!("https://github.com/{org}.png"))
    } else {
        None
    }
}

/// Apply image overrides from vite.config.ts lines 160-171.
fn apply_image_overrides(map: &mut HashMap<&str, MagicLink>) {
    let overrides: Vec<(&str, &str, &str)> = vec![
        ("Yomitan", "https://github.com/yomidevs/yomitan", "https://github.com/yomidevs.png"),
        ("Yomitan Wiki", "https://github.com/yomidevs/yomitan-wiki", "https://github.com/yomidevs.png"),
        ("Vue", "https://github.com/vuejs/core", "https://vuejs.org/logo.svg"),
        ("Nuxt", "https://github.com/nuxt/nuxt", "https://nuxt.com/assets/design-kit/icon-green.svg"),
        ("Vite", "https://github.com/vitejs/vite", "https://vitejs.dev/logo.svg"),
    ];

    for (name, _url, image) in overrides {
        if let Some(link) = map.get_mut(name) {
            link.image_url = Some(image.to_string());
        }
    }
}

/// Process magic links in markdown content.
/// Replaces `{LinkName}` with styled HTML links with image previews.
pub fn process(content: &str) -> String {
    let map = build_links_map();

    // Match {LinkName} but not inside code blocks or existing HTML
    let re = Regex::new(r"\{([^{}]+)\}").unwrap();

    re.replace_all(content, |caps: &regex::Captures| {
        let name = &caps[1];
        if let Some(link) = map.get(name) {
            let img_html = link
                .image_url
                .as_ref()
                .map(|url| format!(r#"<span class="markdown-magic-link-image" style="background-image: url('{url}');"></span>"#))
                .unwrap_or_default();
            format!(
                r#"<a class="markdown-magic-link" href="{}" target="_blank" rel="noopener">{img_html}{name}</a>"#,
                link.url
            )
        } else {
            // Not a known magic link, leave as-is
            caps[0].to_string()
        }
    })
    .to_string()
}
