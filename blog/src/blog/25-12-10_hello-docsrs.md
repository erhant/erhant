# Docs-R-Us

One day I woke up and decided that my blog should live on where my heart is, Rust docs, that is. So here we are, unashamedly piggy-backing [docs.rs](https://docs.rs).

- **Free hosting**: No need to pay for hosting or domain
- **Markdown support**: Write in markdown, rendered beautifully
- **Version control**: Every blog version is preserved
- **Syntax highlighting**: Perfect for technical content
- **Search**: Built-in search functionality

Each blog post is defined using the `blog_post!` macro:

```rust
blog_post! {
    title: "My Post",
    date: "2025-01-01",
    tags: ["rust", "blogging"],
    content: include_str!("posts/my-post.md")
}
```

The macro generates module documentation that appears as the blog post content!

## What's next?

I'll be writing about Rust, programming, and whatever interests me. Stay tuned!
