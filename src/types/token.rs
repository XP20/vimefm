use crate::types::PrefixData;

/// Assignable `errorformat` conversion kinds.
#[derive(Debug, PartialEq, Eq)]
pub enum ConversionKind {
    // TODO(docs, impl): 'isfname', leading "C:" on Windows
    /// %f    file name (finds a string)
    File,
    /// %b    buffer number (finds a number)
    Buffer,
    /// %o    module name (finds a string)
    Module,

    /// %l    line number (finds a number)
    Line,
    /// %e    end line number (finds a number)
    LineEnd,

    /// %c    column number (finds a number representing
    ///       character column of the error, byte index,
    ///       a \t is 1 character column)
    Column,
    /// %v    virtual column number (finds a number representing
    ///       screen column of the error (1 \t is 8 screen columns))
    ColumnVirtual,
    /// %k    end column number (finds a number representing the
    ///       character column of the error, byte index, or
    ///       a number representing screen end column of the
    ///       error if it's used with %v)
    ColumnEnd,
    /// %p    pointer line (finds a sequence of '-', '.', ' ' or
    ///       tabs and uses the length for the column number)
    ColumnPointer,

    /// %t    error type (finds a single character):
    ///           e - error message
    ///           w - warning message
    ///           i - info message
    ///           n - note message
    ErrorType,
    /// %n    error number (finds a number)
    ErrorNum,
    /// %m    error message (finds a string)
    ErrorMsg,

    // TODO(doc): improve docs:
    //            at least 1 char needed, ^$ usage?
    //            https://stackoverflow.com/questions/62276628/need-examples-for-s-in-errorformat-of-vim
    /// %s    search text (finds a string)
    SearchText,

    // TODO: seaprate into MatchKind?
    /// %r    matches the "rest" of a single-line file message %O/P/Q
    ///       regex equivalent of: ".*"
    MatchRest,
    // TODO(impl, lexer): scanf()-like scanset and immediate tokens type
    //                    https://github.com/vim/vim/blob/master/src/quickfix.c#L385
    // NOTE: backward-compatability only
    //       scanf()-like: %*ud, %*3c, %*f, ... unsupported
    /// %*{conv}  any scanf non-assignable conversion
    MatchScanf,
}

// TODO(Token): other modifier and match patterns like %. %# %*[] %\\@=
//
/// An IR token for the Vim `errorformat`.
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// UTF-8 code unit.
    CodeUnit(u8),

    /// A state-dependent
    Prefix(PrefixData),

    /// An assignable conversion.
    Capture(ConversionKind),

    /// A non-assignable conversion.
    Match(ConversionKind),
}
