use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HighlightRequest {
    code: String,
    lang: String,
}

#[derive(Deserialize)]
struct HighlightResponse {
    html: String,
}

/// Highlight all code blocks in HTML by sending them to the Node.js Shiki subprocess.
///
/// Finds `<pre><code class="language-xxx">...</code></pre>` blocks,
/// sends each to `shiki/highlight.mjs`, and replaces with highlighted HTML.
pub fn highlight_code_blocks(html: &str) -> Result<String> {
    let re = Regex::new(
        r#"<pre><code class="language-([^"]+)">([\s\S]*?)</code></pre>"#
    )?;

    let mut result = html.to_string();
    let mut replacements: Vec<(String, String)> = Vec::new();

    for caps in re.captures_iter(html) {
        let full_match = caps[0].to_string();
        let lang = &caps[1];
        let code = html_unescape(&caps[2]);

        match highlight_with_shiki(&code, lang) {
            Ok(highlighted) => {
                replacements.push((full_match, highlighted));
            }
            Err(e) => {
                eprintln!("  Shiki highlight failed for {lang}: {e}");
                // Keep original on failure
            }
        }
    }

    for (original, replacement) in replacements {
        result = result.replace(&original, &replacement);
    }

    Ok(result)
}

/// Find a working Node.js command. Tries `node` first (native), then `node.exe` (WSL → Windows interop).
fn find_node_command() -> &'static str {
    use std::sync::OnceLock;
    static NODE_CMD: OnceLock<&str> = OnceLock::new();
    NODE_CMD.get_or_init(|| {
        if Command::new("node").arg("--version").output().is_ok() {
            "node"
        } else {
            "node.exe"
        }
    })
}

/// Call the Node.js Shiki subprocess to highlight a single code block.
fn highlight_with_shiki(code: &str, lang: &str) -> Result<String> {
    let request = HighlightRequest {
        code: code.to_string(),
        lang: lang.to_string(),
    };

    let input = serde_json::to_string(&request)?;

    let mut child = Command::new(find_node_command())
        .arg("shiki/highlight.mjs")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Shiki process failed: {stderr}");
    }

    let response: HighlightResponse = serde_json::from_slice(&output.stdout)?;
    Ok(response.html)
}

fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}
