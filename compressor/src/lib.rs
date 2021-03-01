pub fn compress(input: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aaabbc() {
        assert_eq!(compress("aaabbc"), "a3bbc")
    }

    #[test]
    fn aaaabbbccc() {
        assert_eq!(compress("aaaabbbccc"), "a4b3c3")
    }

    #[test]
    fn aaabaaa() {
        assert_eq!(compress("aaaabbbccc"), "a3ba3")
    }

    #[test]
    fn empty_string() {
        assert_eq!(compress(""), "")
    }

    #[test]
    fn abcd() {
        assert_eq!(compress("abcd"), "abcd")
    }

    #[test]
    fn aaaaa() {
        assert_eq!(compress("aaaaa"), "a5")
    }

    #[test]
    fn yyyyaaaabbbaaa() {
        assert_eq!(compress("yyyyaaaabbbaaa"), "y4a4b3a3")
    }
}
