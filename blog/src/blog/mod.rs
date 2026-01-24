//! Blog
//!
//! Many of these are migrated from my old blog at [dev.to/erhant](https://dev.to/erhant).

use crate::blog_post;

// auto-generated blog posts from markdown files with frontmatter
// (thanks to build.rs)
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));
