use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let blog_dir = Path::new(&manifest_dir).join("src/blog");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("blog_posts.rs");

    let mut output = String::new();

    // collect and sort markdown files
    let mut md_files: Vec<_> = fs::read_dir(&blog_dir)
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

    md_files.sort_by_key(|entry| entry.path());

    for entry in md_files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let absolute_path = blog_dir.join(filename);

        let content = fs::read_to_string(&path).expect("Failed to read markdown file");

        // parse frontmatter (between --- delimiters)
        if let Some(post) = parse_frontmatter(&content, filename) {
            output.push_str(&format!(
                r#"blog_post! {{
    post: {},
    date: "{}",
    tags: "{}",
    content: include_str!("{}")
}}

"#,
                post.module_name,
                post.date,
                post.tags.join(", "),
                absolute_path.display()
            ));
        }

        // tell Cargo to rerun if any md file changes
        println!("cargo::rerun-if-changed={}", path.display());
    }

    let mut file = fs::File::create(&dest_path).expect("Failed to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write output file");
}

struct BlogPost {
    module_name: String,
    date: String,
    tags: Vec<String>,
}

fn parse_frontmatter(content: &str, filename: &str) -> Option<BlogPost> {
    // check if content starts with HTML-commented frontmatter
    let frontmatter = if content.starts_with("<!--") {
        // find the closing -->
        let end_idx = content.find("-->")?;
        &content[4..end_idx]
    } else if content.starts_with("---") {
        // legacy: support --- delimiters too
        let rest = &content[3..];
        let end_idx = rest.find("---")?;
        &rest[..end_idx]
    } else {
        eprintln!("Warning: {} has no frontmatter, skipping", filename);
        return None;
    };

    let mut date = None;
    let mut tags = Vec::new();
    let mut module_name = None;

    for line in frontmatter.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(value) = line.strip_prefix("date:") {
            date = Some(value.trim().trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("tags:") {
            // Parse tags: [tag1, tag2] or tags: ["tag1", "tag2"]
            let value = value.trim();
            if value.starts_with('[') && value.ends_with(']') {
                let inner = &value[1..value.len() - 1];
                tags = inner
                    .split(',')
                    .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            }
        } else if let Some(value) = line.strip_prefix("post:") {
            module_name = Some(value.trim().trim_matches('"').to_string());
        }
    }

    // derive module name from filename if not specified
    // e.g., "22-01-01_euclid-mullin.md" -> "euclid_mullin"
    let module_name = module_name.unwrap_or_else(|| {
        let stem = filename.strip_suffix(".md").unwrap_or(filename);
        // remove date prefix (YY-MM-DD_)
        let name = if stem.len() > 10 && stem.chars().nth(8) == Some('_') {
            &stem[9..]
        } else {
            stem
        };
        // convert hyphens to underscores for valid Rust identifier
        name.replace('-', "_")
    });

    let date = date.unwrap_or_else(|| {
        // try to extract date from filename (YY-MM-DD_...)
        if filename.len() >= 8 {
            let year = &filename[0..2];
            let month = &filename[3..5];
            let day = &filename[6..8];
            format!("20{}-{}-{}", year, month, day)
        } else {
            "1970-01-01".to_string()
        }
    });

    Some(BlogPost {
        module_name,
        date,
        tags,
    })
}
