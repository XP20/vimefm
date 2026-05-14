pub mod prefix;
pub mod token;

pub use prefix::*;
pub use token::*;

// TODO(docs): format string blanks after the comma separator are ignored
//
/// An IR for a Vim `errorformat` string
///
/// See `:help errorformat` in Vim for the format specification.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct ErrorFormat {
    /// Packed prefix and flag of a Vim `errorformat` string.
    pub prefix: Prefix,

    /// IR tokens of a Vim `errorformat` string.
    pub tokens: Vec<Token>,
}
