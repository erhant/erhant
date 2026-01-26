//! My blog posts
//!
//! Pre-2026 blogs are migrated from my old blog at [dev.to/erhant](https://dev.to/erhant).
//!
#![doc = include_str!(concat!(env!("OUT_DIR"), "/blog_toc.md"))]

// auto-generated blog posts from markdown files with frontmatter
// (thanks to build.rs)
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));
