#[cfg(test)]
mod test_utils {
    use crate::debug::efm_to_string;
    use crate::types::Token;

    #[inline]
    pub fn lex_debug(input: Vec<Vec<Token>>) -> String {
        input
            .iter()
            .map(efm_to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    #[inline]
    pub fn efm_trim(input: &str) -> String {
        let mut output = String::new();
        let mut iter = input.chars();

        while let Some(c) = iter.next() {
            output.push(c);
            match c {
                ',' => {
                    iter.find(|&c_next| c_next != ' ')
                        .inspect(|&x| output.push(x));
                }
                '\\' => {
                    iter.next().inspect(|&x| output.push(x));
                }
                _ => {}
            };
        }
        return output;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;

    use super::test_utils;

    #[test]
    fn test_simple() {
        const EFM: &str = "%f:%l:%c: %m";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), test_utils::efm_trim(EFM));
    }

    #[test]
    fn test_simple_multi() {
        const EFM: &str =
            r"%f: line %l\, col %c\, %troror - %m,    %f: line %l\, col %c\, %tarning - %m";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), test_utils::efm_trim(EFM));
    }

    #[test]
    fn test_simple_multiline() {
        const EFM: &str = r"%EError %n,%Cline %l,%Ccolumn %c,%Z%m";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), test_utils::efm_trim(EFM));
    }

    #[test]
    fn test_prefix() {
        const EFM: &str = "%-G%.%#";

        let output = lex(EFM).unwrap();
        assert_eq!(test_utils::lex_debug(output), test_utils::efm_trim(EFM));
    }
}
