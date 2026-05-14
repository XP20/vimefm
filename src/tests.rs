#[cfg(test)]
mod test_utils {
    use crate::debug::efm_to_string;
    use crate::types::ErrorFormat;

    #[inline]
    pub fn lex_debug(input: Vec<ErrorFormat>) -> String {
        input
            .iter()
            .map(efm_to_string)
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;

    use super::test_utils;

    #[test]
    fn test_simple() {
        const EFM: &str = "%f:%l:%c: %m";
        const EXPECTED: &str = "%f:%l:%c: %m";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), EXPECTED);
    }

    #[test]
    fn test_simple_multi() {
        const EFM: &str =
            r"%f: line %l\, col %c\, %troror - %m,    %f: line %l\, col %c\, %tarning - %m";
        const EXPECTED: &str =
            r"%f: line %l\, col %c\, %troror - %m,%f: line %l\, col %c\, %tarning - %m";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), EXPECTED);
    }

    #[test]
    fn test_simple_multiline() {
        const EFM: &str = r"%EError %n,%Cline %l,%Ccolumn %c,%Z%m";
        const EXPECTED: &str = r"%EError %n,%Cline %l,%Ccolumn %c,%Z%m";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), EXPECTED);
    }

    #[test]
    fn test_prefix() {
        const EFM: &str = "%-G%.%#";
        const EXPECTED: &str = "%-G%.%#";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), EXPECTED);
    }
}
