#![feature(once_cell)]

use proc_macro::TokenStream;

mod renderer;

/// Render RainbowML.
///
/// # Arguments
///
/// * `stream`:
///
/// returns: String
///
/// # Examples
///
/// ```
/// use rainbow_macro::rainbow_rml;
/// rainbow_rml!("<keyword>Hello, world!</keyword>", "one-dark.rmb");
/// ```
#[proc_macro]
pub fn rainbow_rml(stream: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

/// Render Source Code.
///
/// # Arguments
///
/// * `stream`:
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
///
/// ```
#[proc_macro]
pub fn rainbow_code(stream: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}


/// Convert SourceCode to RainbowML.
///
/// # Arguments
///
/// * `stream`:
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
///
/// ```
#[proc_macro]
pub fn code_to_rml(stream: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}