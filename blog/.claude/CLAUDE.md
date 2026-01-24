# Blog on docs.rs

This is a personal blog hosted on [docs.rs](https://docs.rs/erhant), using Rust docstrings as blog posts.

## How It Works

Blog posts are markdown files in `src/blog/` with HTML-commented frontmatter. A build script (`build.rs`) automatically:

1. Scans `src/blog/*.md` for markdown files
2. Parses frontmatter for metadata (date, tags)
3. Generates `blog_post!` macro calls
4. Numbers posts for reverse-chronological sorting on docs.rs (`n1_` = newest)

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

- `date`: Publication date (required, or derived from filename)
- `tags`: Array of tags
- `post`: Optional custom module name (defaults to slug from filename)

HTML comments are used so the frontmatter is hidden in rendered docs.

## Generated Output

The build script generates `blog_posts.rs` in `OUT_DIR` with entries like:

```rust
blog_post! {
    post: n1_hello_docsrs,
    date: "2025-01-01",
    tags: "programming",
    content: include_str!("/path/to/src/blog/25-12-10_hello-docsrs.md")
}
```

Posts are numbered `n1_`, `n2_`, etc. (newest first) for proper sorting on docs.rs.

## Key Files

- `build.rs` - Generates blog post code from markdown files
- `src/macros.rs` - Defines the `blog_post!` macro
- `src/blog/mod.rs` - Includes the generated blog posts
- `src/blog/*.md` - Blog post content
