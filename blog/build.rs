use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let blog_dir = Path::new(&manifest_dir).join("src/blog");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let dest_path = out_path.join("blog_posts.rs");

    // collect markdown files
    let md_files: Vec<_> = fs::read_dir(&blog_dir)
        .expect("Failed to read blog directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
        .collect();

    // parse all posts, transform content, and collect with their output paths
    let mut posts: Vec<_> = md_files
        .iter()
        .map(|entry| {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("{}: failed to read file: {}", filename, e));
            let post = parse_frontmatter(&content, filename);

            // transform mermaid blocks and write to OUT_DIR
            let transformed = transform_mermaid_blocks(&content);
            let out_file_path = out_path.join(filename);
            fs::write(&out_file_path, &transformed).unwrap_or_else(|e| {
                panic!("{}: failed to write transformed file: {}", filename, e)
            });

            (post, out_file_path)
        })
        .collect();

    // remove work-in-progress posts
    posts.retain(|p| !p.0.wip);

    // sort by date descending (newest first)
    posts.sort_by(|a, b| b.0.date.cmp(&a.0.date));

    // generate table of contents markdown file
    let toc_path = out_path.join("blog_toc.md");
    let mut toc = String::new();
    for (post, _) in posts.iter() {
        toc.push_str(&format!(
            "- **{}** [{}]({})  — {}\n",
            post.date, post.title, post.module_name, post.summary
        ));
    }
    fs::write(&toc_path, &toc).expect("Failed to write blog_toc.md");

    // generate output with numbered prefixes (1 = newest)
    let mut output = String::new();

    for (post, transformed_path) in posts.iter() {
        let tags = post.tags.join(", ");

        // generate the module definition
        output.push_str(&format!(
            r#"#[doc = "**Published:** {date} | _{tags}_"]
#[doc = ""]
#[doc = include_str!("{path}")]
pub mod {name} {{}}

"#,
            date = post.date,
            tags = tags,
            path = transformed_path.display(),
            name = post.module_name,
        ));
    }

    // tell Cargo to rerun if the blog directory changes (new/removed/renamed files)
    println!("cargo::rerun-if-changed={}", blog_dir.display());
    // also rerun if any individual md file changes (content edits)
    for entry in &md_files {
        println!("cargo::rerun-if-changed={}", entry.path().display());
    }

    let mut file = fs::File::create(&dest_path).expect("Failed to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write output file");
}

struct BlogPost {
    module_name: String,
    title: String,
    date: String,
    tags: Vec<String>,
    summary: String,
    wip: bool,
}

fn parse_frontmatter(content: &str, filename: &str) -> BlogPost {
    // check if content starts with HTML-commented frontmatter
    let frontmatter = if content.starts_with("<!--") {
        let end_idx = content
            .find("-->")
            .unwrap_or_else(|| panic!("{}: unclosed frontmatter comment", filename));
        &content[4..end_idx]
    } else if content.starts_with("---") {
        // legacy: support --- delimiters too
        let rest = &content[3..];
        let end_idx = rest
            .find("---")
            .unwrap_or_else(|| panic!("{}: unclosed frontmatter delimiter", filename));
        &rest[..end_idx]
    } else {
        panic!("{}: missing frontmatter", filename);
    };

    let mut date = None;
    let mut tags = None;
    let mut module_name = None;
    let mut title = None;
    let mut summary = None;
    let mut wip = false;

    for line in frontmatter.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(value) = line.strip_prefix("date:") {
            date = Some(value.trim().trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("tags:") {
            // parse tags: [tag1, tag2] or tags: ["tag1", "tag2"]
            let value = value.trim();
            if value.starts_with('[') && value.ends_with(']') {
                let inner = &value[1..value.len() - 1];
                tags = Some(
                    inner
                        .split(',')
                        .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|t| !t.is_empty())
                        .collect(),
                );
            }
        } else if let Some(value) = line.strip_prefix("post:") {
            module_name = Some(value.trim().trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("title:") {
            title = Some(value.trim().trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("summary:") {
            summary = Some(value.trim().trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("wip:") {
            wip = value.trim() == "true";
        }
    }

    let date = date.unwrap_or_else(|| panic!("{}: missing 'date' in frontmatter", filename));
    let tags = tags.unwrap_or_else(|| panic!("{}: missing 'tags' in frontmatter", filename));
    let title = title.unwrap_or_else(|| panic!("{}: missing 'title' in frontmatter", filename));
    let summary =
        summary.unwrap_or_else(|| panic!("{}: missing 'summary' in frontmatter", filename));

    // derive module name from filename if not specified
    // e.g., "euclid-mullin.md" -> "euclid_mullin"
    let module_name = module_name.unwrap_or_else(|| {
        let stem = filename.strip_suffix(".md").unwrap_or(filename);
        // convert hyphens to underscores for valid Rust identifier
        stem.replace('-', "_")
    });

    BlogPost {
        module_name,
        title,
        date,
        tags,
        summary,
        wip,
    }
}

/// Transforms ```mermaid code blocks into HTML that renders via Mermaid.js
fn transform_mermaid_blocks(content: &str) -> String {
    let mut result = String::new();
    let mut remaining = content;

    while let Some(start) = remaining.find("```mermaid") {
        // add everything before the mermaid block
        result.push_str(&remaining[..start]);

        // find the end of the opening fence (after "```mermaid")
        let after_fence = &remaining[start + 10..];
        let content_start = after_fence.find('\n').map(|i| i + 1).unwrap_or(0);
        let mermaid_content_start = &after_fence[content_start..];

        // find the closing ```
        if let Some(end) = mermaid_content_start.find("\n```") {
            let diagram = &mermaid_content_start[..end];

            // generate HTML for mermaid
            // inspired from: https://github.com/glueball/simple-mermaid
            result.push_str(
                "<pre class=\"mermaid\" style=\"text-align:center;background:transparent;\">\n",
            );
            result.push_str(diagram);
            result.push_str("\n</pre>");
            result.push_str("<script type=\"module\">");
            result.push_str("import mermaid from \"https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs\";");
            result.push_str("var doc_theme = localStorage.getItem(\"rustdoc-theme\");");
            result.push_str("if (doc_theme === \"dark\" || doc_theme === \"ayu\") mermaid.initialize({theme: \"dark\"});");
            result.push_str("</script>");

            // move past the closing ```
            let close_fence_end = content_start + end + 4; // +4 for "\n```"
            remaining = &after_fence[close_fence_end..];
        } else {
            // malformed block, keep as-is
            result.push_str(&remaining[start..start + 10]);
            remaining = &remaining[start + 10..];
        }
    }

    // add any remaining content
    result.push_str(remaining);
    result
}
