use crate::errors::*;
use crate::types::*;

fn lex_conversion<'a, I>(
    b_first: Option<I::Item>,
    format: &mut ErrorFormat,
    iter: &mut I,
) -> Result<Option<I::Item>, EfmError>
where
    I: Iterator<Item = &'a u8>,
{
    let token = match b_first {
        Some(&b'f') => Token::Capture(ConversionKind::File),
        Some(&b'b') => Token::Capture(ConversionKind::Buffer),
        Some(&b'o') => Token::Capture(ConversionKind::Module),
        Some(&b'l') => Token::Capture(ConversionKind::Line),
        Some(&b'e') => Token::Capture(ConversionKind::LineEnd),
        Some(&b'c') => Token::Capture(ConversionKind::Column),
        Some(&b'v') => Token::Capture(ConversionKind::ColumnVirtual),
        Some(&b'k') => Token::Capture(ConversionKind::ColumnEnd),
        Some(&b'p') => Token::Capture(ConversionKind::ColumnPointer),
        Some(&b't') => Token::Capture(ConversionKind::ErrorType),
        Some(&b'n') => Token::Capture(ConversionKind::ErrorNum),
        Some(&b'm') => Token::Capture(ConversionKind::ErrorMsg),
        Some(&b'r') => Token::Match(ConversionKind::MatchRest),
        Some(&b's') => Token::Match(ConversionKind::MatchSearch),
        // TODO: scanf()-like matching
        // probably here calling on a function to greedily extract the full scanf() tokens
        // and use new Token::Scanf(ScanfKind) for scanset and vim-regexp
        // lex_scanset and lex_vimregexp further split
        Some(&b'*') => todo!("scanf()-like matching"),
        Some(&b'%') => Token::CodeUnit(b'%'),
        Some(&c) => {
            return Err(EfmError::Syntax(format!(
                "{}: Unknown conversion '%{}'",
                VimError::EInvalidChrInFormatString,
                c as char
            )));
        }
        None => {
            return Err(EfmError::Syntax(format!(
                "{}: Trailing percent, use %% for a literal '%'",
                VimError::EInvalidChrInFormatString,
            )));
        }
    };

    format.tokens.push(token);
    Ok(iter.next())
}

fn lex_backslash<'a, I>(
    b_first: Option<I::Item>,
    format: &mut ErrorFormat,
    iter: &mut I,
) -> Result<Option<I::Item>, EfmError>
where
    I: Iterator<Item = &'a u8>,
{
    let token: Token = match b_first {
        Some(&b'\\') => Token::CodeUnit(b'\\'),
        Some(&b',') => Token::CodeUnit(b','),
        Some(&b' ') => Token::CodeUnit(b' '),
        Some(&c) => {
            return Err(EfmError::Syntax(format!(
                r"Unknown backslash escape '\{}', use \\ for a literal '\'",
                c as char
            )));
        }
        None => {
            return Err(EfmError::Syntax(String::from(
                r"Trailing backslash, use \\ for a literal '\'",
            )));
        }
    };

    format.tokens.push(token);
    Ok(iter.next())
}

fn capture_prefix<'a, I>(
    b_first: Option<I::Item>,
    format: &mut ErrorFormat,
    iter: &mut I,
) -> Result<Option<I::Item>, EfmError>
where
    I: Iterator<Item = &'a u8>,
{
    let mut b_next = b_first;

    // Handle flag and advance iterator
    let flag: Option<char> = match b_next {
        Some(&b'-') => Some('-'),
        Some(&b'+') => Some('+'),
        _ => None,
    };
    if let Some(f) = flag {
        format.prefix.set_flag(Some(f == '+'));
        b_next = iter.next();
    }

    let prefix = match b_next {
        Some(&b'E') => PrefixKind::MultilineError,
        Some(&b'W') => PrefixKind::MultilineWarn,
        Some(&b'I') => PrefixKind::MultilineInfo,
        Some(&b'N') => PrefixKind::MultilineNote,
        Some(&b'A') => PrefixKind::MultilineAny,
        Some(&b'C') => PrefixKind::MultilineContinue,
        Some(&b'Z') => PrefixKind::MultilineEnd,
        Some(&b'>') => PrefixKind::PatternContinue,
        Some(&b'D') => PrefixKind::DirectoryEnter,
        Some(&b'X') => PrefixKind::DirectoryLeave,
        Some(&b'O') => PrefixKind::StackOver,
        Some(&b'P') => PrefixKind::StackPush,
        Some(&b'Q') => PrefixKind::StackPop,
        Some(&b'G') => PrefixKind::General,
        Some(&b_other) => {
            return match flag {
                // Flag must be followed by prefix, so invalid format string
                Some(f) => Err(EfmError::Syntax(format!(
                    "{}: Invalid format string prefix '%{f}{p}', maybe you meant '%{f}G{p}'?",
                    VimError::EInvalidChrInFormatStringPrefix,
                    p = b_other as char,
                ))),

                // No flag, byte is not a prefix but might be a conversion
                None => lex_conversion(b_next, format, iter),
            };
        }
        None => {
            return match flag {
                Some(f) => Err(EfmError::Syntax(format!(
                    "{}: Expected prefix to follow flag, maybe you meant '%{f}G'?",
                    VimError::EInvalidChrInFormatStringPrefix,
                ))),
                None => Err(EfmError::Syntax(format!(
                    "{}: Trailing percent, use %% for a literal '%'",
                    VimError::EInvalidChrInFormatString
                ))),
            };
        }
    };

    format.prefix.set_prefix(Some(prefix));
    Ok(iter.next())
}

/// Tokenizes the given Vim `errorformat` string
/// across ErrorFormat IR structs for the interpreter.
///
/// See `:help errorformat` in Vim for the format specification.
pub fn lex(errorformat: &str) -> Result<Vec<ErrorFormat>, EfmError> {
    let mut formats: Vec<ErrorFormat> = Vec::new();
    let mut iter = errorformat.as_bytes().iter();

    let mut format = ErrorFormat::default();
    let mut b_next = iter.next();

    // Handle format string prefix (e.g. '%-G')
    match b_next {
        Some(&b'%') => b_next = capture_prefix(iter.next(), &mut format, &mut iter)?,
        None => {
            return Err(EfmError::Syntax(format!(
                "{}: 'errorformat' contains no pattern",
                VimError::EErrorformatContainsNoPattern
            )));
        }
        _ => {}
    };

    // Lex format string bytes into tokens
    while let Some(&c) = b_next {
        b_next = match c {
            b'%' => lex_conversion(iter.next(), &mut format, &mut iter),
            b'\\' => lex_backslash(iter.next(), &mut format, &mut iter),
            b',' => {
                if format == ErrorFormat::default() {
                    return Err(EfmError::Syntax(format!(
                        "{}: Some 'errorformat' entry contains no pattern",
                        VimError::EErrorformatContainsNoPattern
                    )));
                }
                // Push entry and start new ErrorFormat
                formats.push(std::mem::take(&mut format));

                // Skip leading whitespace after comma
                Ok(iter.find(|&&b| b != b' '))
            }
            _ => {
                format.tokens.push(Token::CodeUnit(c));
                Ok(iter.next())
            }
        }?;
    }

    formats.push(format);
    Ok(formats)
}
