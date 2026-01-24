/// Creates a blog post module with metadata and content.
///
/// The resulting docstring supports markdown, code highlighting ([`doctored`]), and math rendering (KaTeX).
///
///
/// $$
/// f(x) = \int_{-\infty}^\infty
///   \hat f(\xi)e^{2 \pi i \xi x}
///   d\xi
/// $$
///
///
/// ```mmd
/// sequenceDiagram
///    autonumber
///
///    participant U as User
///    participant FE as Frontend (Bun)
///    participant BE as Backend API (Bun)
///    participant ST as Stripe Checkout
///    participant WH as Stripe Webhook Endpoint
///    participant DB as Database
///
///    %% Step 1: User selects credits
///    U->>FE: Selects credit package (e.g. 300 credits)
///    FE->>BE: POST /create-checkout-session { credits }   
/// ```
/// # Examples
///
/// ```rs
/// blog_post! {
///     title: "My First Post",
///     date: "2025-01-15",
///     tags: ["rust", "meta"],
///     content: include_str!("posts/my-first-post.md")
/// }
/// ```
#[macro_export]
macro_rules! blog_post {
    (
        post: $post:ident,
        date: $date:expr,
        tags: [$($tag:expr),* $(,)?],
        content: $content:expr
    ) => {
        #[doctored::doctored] // use https://docs.rs/doctored/
        #[doc(highlight)] // add code higlighting via doctored
        #[doc = concat!("**Published:** ", $date)]
        #[doc = ""]
        #[doc = concat!("**Tags:** ", $($tag, ", "),*)]
        #[doc = ""]
        #[doc = "---"]
        #[doc = ""]
        #[doc = $content]
        pub mod $post {}
    };
}
