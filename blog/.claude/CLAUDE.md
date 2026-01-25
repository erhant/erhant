# Blog on docs.rs

This is a personal blog hosted on [docs.rs](https://docs.rs/erhant), using Rust docstrings as blog posts.

## How It Works

Blog posts are markdown files in `src/blog/` with HTML-commented frontmatter. The build script (`build.rs`) automatically:

1. Scans `src/blog/*.md` for markdown files
2. Parses frontmatter for metadata (date, tags)
3. Transforms mermaid code blocks into HTML
4. Generates module definitions with doc attributes
5. Numbers posts for reverse-chronological sorting on docs.rs (`n1_` = newest)

## Adding a New Post

1. Create a markdown file in `src/blog/` with naming convention `YY-MM-DD_slug.md`
2. Add HTML-commented frontmatter at the top:

```markdown
<!--
date: "YYYY-MM-DD"
tags: [tag1, tag2]
-->

# Post Title

Your content here...
```

3. Build with `cargo build` - the post is automatically included

## Frontmatter Format

```markdown
<!--
date: "2025-01-15"
tags: [rust, programming]
post: optional_custom_module_name
-->
```

- `date`: Publication date (required)
- `tags`: Array of tags (required)
- `post`: Optional custom module name (defaults to slug from filename)

HTML comments are used so the frontmatter is hidden in rendered docs.

## Generated Output

The build script generates `blog_posts.rs` in `OUT_DIR` with module definitions:

```rust
#[doc = "**Published:** 2025-01-01 | **Tags:** rust, meta"]
#[doc = ""]
#[doc = "---"]
#[doc = ""]
#[doc = include_str!("/path/to/out_dir/25-12-10_hello-docsrs.md")]
pub mod n1_hello_docsrs {}
```

Posts are numbered `n1_`, `n2_`, etc. (newest first) for proper sorting on docs.rs.

## Architecture & Structure

```
blog/
├── build.rs              # All the magic: parse, transform, generate
├── Cargo.toml            # Package config (edition 2024, MIT license)
├── styleheader.html     # KaTeX header for math rendering & code highlighting for docs.rs
├── src/
│   ├── lib.rs            # Crate root, exports `about` and `blog` modules
│   ├── about/
│   │   ├── mod.rs        # About module with README embed
│   │   └── README.md     # Personal profile/links
│   └── blog/
│       ├── mod.rs        # Blog module, includes generated blog_posts.rs
│       └── *.md          # Blog posts (YY-MM-DD_slug.md format)
└── .claude/
    └── CLAUDE.md         # This file
```

## Key Components

### build.rs

The build script handles everything:

- Scans `src/blog/*.md` for posts
- Parses HTML-commented frontmatter (`<!-- ... -->`) or legacy `---` delimiters
- Extracts `date`, `tags`, and optional `post` (module name)
- Transforms ` ```mermaid ` code blocks into HTML with Mermaid.js CDN
- Sorts posts by date descending (newest first)
- Generates numbered module names (`n1_`, `n2_`, ...) for docs.rs ordering
- Writes transformed markdown to `OUT_DIR`
- Outputs `blog_posts.rs` with full module definitions

### docs.rs Configuration

- Custom rustdoc args include `styleheader.html` for KaTeX math rendering & code highlighting

## Development Notes

- **Adding posts**: Create `src/blog/YY-MM-DD_slug.md` with frontmatter, then `cargo build`
- **Mermaid diagrams**: Use ` ```mermaid ` code blocks - transformed automatically
- **Naming**: Module names derived from filename slug (hyphens → underscores)
- **Ordering**: Numeric prefixes ensure newest posts appear first on docs.rs
- **Publishing**: `cargo publish` pushes to crates.io, docs.rs builds automatically
- **Edition**: Uses Rust 2024 edition

## Testing Strategy

This project is documentation-only (no executable code), so testing is minimal:

- **Build verification**: `cargo build` ensures posts parse correctly
- **Doc generation**: `cargo doc --open` previews rendered output locally
- **CI**: Publishing to crates.io triggers docs.rs build for final verification
