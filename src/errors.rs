/// Vim `errorformat` error numbers.
///
/// See `:help errorformat` in Vim for the format specification.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum VimError {
    /// E372: Too many %{c} in format string
    ETooManyChrInFormatString = 372,

    /// E373: Unexpected %{c} in format string
    EUnexpectedChrInFormatStr = 373,

    /// E374: Missing ] in format string
    EMissingRsbInFormatString = 374,

    /// E375: Unsupported %{c} in format string
    EUnsupportedChrInFormatString = 375,

    /// E376: Invalid %{c} in format string prefix
    EInvalidChrInFormatStringPrefix = 376,

    /// E377: Invalid %{c} in format string
    EInvalidChrInFormatString = 377,

    /// E378: 'errorformat' contains no pattern
    EErrorformatContainsNoPattern = 378,

    /// E379: Missing or empty directory name
    EMissingOrEmptyDirectoryName = 379,
}

impl std::fmt::Display for VimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{}", *self as u16)
    }
}

#[derive(Debug)]
pub enum EfmError {
    /// Occurs when lexical analysis fails for a Vim `errorformat` string.
    Syntax(String),
}

impl std::fmt::Display for EfmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EfmError::Syntax(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for EfmError {}
