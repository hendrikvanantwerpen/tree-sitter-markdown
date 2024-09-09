//! This crate provides Markdown language support for the [tree-sitter][] parsing library.
//!
//! It contains two grammars: [`language`] to parse the block structure of markdown documents and
//! [`inline_language`] to parse inline content.
//!
//! It also supplies [`MarkdownParser`] as a convenience wrapper around the two grammars.
//! [`MarkdownParser::parse`] returns a [`MarkdownTree`] instread of a [`Tree`][Tree]. This struct
//! contains a block tree and an inline tree for each node in the block tree that has inline
//! content
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [Tree]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Tree.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_markdown() -> *const ();
    fn tree_sitter_markdown_inline() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for the block grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_markdown) };

/// The tree-sitter [`LanguageFn`] for the inline grammar.
pub const INLINE_LANGUAGE: LanguageFn =
    unsafe { LanguageFn::from_raw(tree_sitter_markdown_inline) };

pub const HIGHLIGHT_QUERY_BLOCK: &str =
    include_str!("../../tree-sitter-markdown/queries/highlights.scm");
pub const INJECTION_QUERY_BLOCK: &str =
    include_str!("../../tree-sitter-markdown/queries/injections.scm");
pub const HIGHLIGHT_QUERY_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/queries/highlights.scm");
pub const INJECTION_QUERY_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/queries/injections.scm");

/// The content of the [`node-types.json`][] file for the block grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_BLOCK: &str = include_str!("../../tree-sitter-markdown/src/node-types.json");

/// The content of the [`node-types.json`][] file for the inline grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_INLINE: &str =
    include_str!("../../tree-sitter-markdown-inline/src/node-types.json");

#[cfg(feature = "parser")]
mod parser;

#[cfg(feature = "parser")]
pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Markdown language");
        let mut inline_parser = tree_sitter::Parser::new();
        inline_parser
            .set_language(&INLINE_LANGUAGE.into())
            .expect("Error loading Markdown language");
    }
}
