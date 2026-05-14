/// The following uppercase conversion kinds specify the type of special `errorformat`
/// format strings. At most one of them may be given as a prefix at the beginning
/// of a single comma-separated format pattern.
///
/// The prefix may be preceeded by a flag:
/// %-    do not include the matching multi-line in any output
/// %+    include the whole matching line in the %m error string
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum PrefixKind {
    /// %E    start of a multi-line error message
    MultilineError = 0,
    /// %W    start of a multi-line warning message
    MultilineWarn = 1,
    /// %I    start of a multi-line informational message
    MultilineInfo = 2,
    /// %N    start of a multi-line note message
    MultilineNote = 3,
    /// %A    start of a multi-line message (unspecified type)
    MultilineAny = 4,
    // TODO(impl): fail on non-multiline flag
    /// %C    continuation of a multi-line message
    MultilineContinue = 5,
    /// %Z    end of a multi-line message
    MultilineEnd = 6,

    // TODO(impl): better understand usage of %>
    /// %>    for the next line start with current pattern again
    PatternContinue = 7,

    // TODO(docs): When defining an "enter directory" or "leave directory" format,
    //             the "%D" or "%X" has to be given at the start of that substring.
    /// %D    "enter directory" format string;
    ///       expects a following %f that finds the directory name
    DirectoryEnter = 8,
    /// %X    "leave directory" format string;
    ///       expects a following %f
    DirectoryLeave = 9,

    // TEMP: %O (single-line file skip %f writing)
    /// %O    todo
    StackOver = 10,
    // TEMP: %P (single-line file push %f stack) %+P ?
    /// %P    todo
    StackPush = 11,
    // TEMP: %Q (single-line file pop stack)
    /// %Q    todo
    StackPop = 12,

    /// %G    this prefix is only useful in combination with '+' or '-'. It
    ///       parses over lines containing general information like compiler
    ///       version strings or other headers that can be skipped.
    ///
    /// Examples:
    /// %-G   ignore this message
    /// %+G   general message
    General = 13,
}

/// A byte that stores the prefix with the flag
/// of a Vim `errorformat` string.
///
/// Packing structure:
/// bit    7: 1 if flag is present
/// bit    6: 1 if flag is %+, 0 if %-
/// bits 5-0: FlagKind enum discriminant (u6)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prefix(u8);

impl Default for Prefix {
    #[inline(always)]
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl Prefix {
    /// Creates the Prefix packed struct.
    /// Unless flag is set, initializes with 0.
    /// Unless prefix is set, initializes with (u8) 63.
    #[inline(always)]
    pub const fn new(flag: Option<bool>, prefix: Option<PrefixKind>) -> Self {
        Self(
            0 | match flag {
                Some(f) => 0b1000_0000 | ((f as u8) << 6),
                None => 0b0000_0000,
            } | match prefix {
                Some(p) => p as u8,
                None => 0b0011_1111,
            },
        )
    }

    /// Unpacks the flag portion of Prefix.
    ///
    /// If flag is unset returns None.
    /// Otherwise returns Some(bool) where bool:
    /// is true  for flag %+
    /// is false for flag %-
    ///
    /// %-    do not include the matching multi-line in any output
    /// %+    include the whole matching line in the %m error string
    #[inline(always)]
    pub fn get_flag(self: &Self) -> Option<bool> {
        match self.0 & 0b1000_0000 {
            0 => None,
            _ => Some((self.0 & 0b0100_0000) != 0),
        }
    }

    /// Sets the flag portion of packed Prefix.
    #[inline(always)]
    pub fn set_flag(self: &mut Self, flag: Option<bool>) {
        self.0 &= 0b0011_1111;
        self.0 |= match flag {
            Some(f) => 0b1000_0000 | ((f as u8) << 6),
            None => 0b0000_0000,
        };
    }

    /// Unpacks the prefix kind enum portion of Prefix.
    ///
    /// If prefix is unset returns None.
    /// Otherwise returns Some(PrefixKind).
    #[inline(always)]
    pub fn get_prefix(self: &Self) -> Option<PrefixKind> {
        match self.0 & 0b0011_1111 {
            00 => Some(PrefixKind::MultilineError),
            01 => Some(PrefixKind::MultilineWarn),
            02 => Some(PrefixKind::MultilineInfo),
            03 => Some(PrefixKind::MultilineNote),
            04 => Some(PrefixKind::MultilineAny),
            05 => Some(PrefixKind::MultilineContinue),
            06 => Some(PrefixKind::MultilineEnd),
            07 => Some(PrefixKind::PatternContinue),
            08 => Some(PrefixKind::DirectoryEnter),
            09 => Some(PrefixKind::DirectoryLeave),
            10 => Some(PrefixKind::StackOver),
            11 => Some(PrefixKind::StackPush),
            12 => Some(PrefixKind::StackPop),
            13 => Some(PrefixKind::General),
            _ => None,
        }
    }

    /// Sets the prefix kind enum portion of packed Prefix.
    #[inline(always)]
    pub fn set_prefix(self: &mut Self, prefix: Option<PrefixKind>) {
        self.0 &= 0b1100_0000;
        self.0 |= match prefix {
            Some(p) => p as u8,
            None => 0b0011_1111,
        }
    }
}
