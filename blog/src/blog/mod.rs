//! Blog
//!
//! Many of these are migrated from my old blog at https://dev.to/erhant.

use crate::blog_post;

blog_post! {
    post: hello_docs_rs,
    date: "2025-01-01",
    tags: ["meta", "rust", "blogging"],
    content: include_str!("25-12-10_hello-docsrs.md")
}

blog_post! {
    post: euclid_mullin_sequence,
    date: "2022-01-01",
    tags: ["math"],
    content: include_str!("22-01-01_euclid-mullin.md")
}

blog_post! {
    post: evm_puzzles,
    date: "2023-01-24",
    tags: ["web3", "evm", "puzzles"],
    content: include_str!("23-01-24_evm-puzzles.md")
}
