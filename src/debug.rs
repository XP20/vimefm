use crate::types::*;

// TODO(doc): function
pub fn efm_to_string(errorformat: &Vec<Token>) -> String {
    let mut output = String::new();
    let mut iter = errorformat.iter();

    while let Some(token) = iter.next() {
        let str = match token {
            Token::CodeUnit(b) => match b {
                &b'\\' => String::from(r"\\"),
                &b',' => String::from(r"\,"),
                // TODO(impl): UTF-8 parsing
                &b_other => (b_other as char).to_string(),
            },
            Token::Capture(t) | Token::Match(t) => match t {
                &ConversionKind::File => "%f",
                &ConversionKind::Buffer => "%b",
                &ConversionKind::Module => "%o",
                &ConversionKind::Line => "%l",
                &ConversionKind::LineEnd => "%e",
                &ConversionKind::Column => "%c",
                &ConversionKind::ColumnVirtual => "%v",
                &ConversionKind::ColumnEnd => "%k",
                &ConversionKind::ColumnPointer => "%k",
                &ConversionKind::ErrorType => "%t",
                &ConversionKind::ErrorNum => "%n",
                &ConversionKind::ErrorMsg => "%m",
                &ConversionKind::MatchRest => "%r",
                &ConversionKind::SearchText => "%s",
                &ConversionKind::MatchScanf => "%f",
            }
            .to_string(),
            Token::Prefix(d) => format!(
                "%{f}{p}",
                f = match d.get_flag() {
                    None => "",
                    Some(true) => "+",
                    Some(false) => "-",
                },
                p = match d.get_prefix().expect("present Prefix token to be valid") {
                    PrefixKind::MultilineError => 'E',
                    PrefixKind::MultilineWarn => 'W',
                    PrefixKind::MultilineInfo => 'I',
                    PrefixKind::MultilineNote => 'N',
                    PrefixKind::MultilineAny => 'A',
                    PrefixKind::MultilineContinue => 'C',
                    PrefixKind::MultilineEnd => 'Z',
                    PrefixKind::PatternContinue => '>',
                    PrefixKind::DirectoryEnter => 'D',
                    PrefixKind::DirectoryLeave => 'X',
                    PrefixKind::StackOver => 'O',
                    PrefixKind::StackPush => 'P',
                    PrefixKind::StackPop => 'Q',
                    PrefixKind::General => 'G',
                }
            ),
        };
        output.push_str(str.as_str());
    }

    return output;
}
