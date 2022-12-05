pub fn strip_quotes(val: &str) -> &str {
    let mut chars = val.chars();
    if val.starts_with('"') {
        chars.next();
    }
    if val.ends_with('"') {
        chars.next_back();
    }
    return chars.as_str();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_quotes_strips_quotes() {
        assert_eq!(strip_quotes("\"hello\""), "hello");
    }

    #[test]
    fn unquoted_strings_remain_the_same() {
        assert_eq!(strip_quotes("hello"), "hello");
    }
}
