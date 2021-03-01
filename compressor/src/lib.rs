pub fn compress(input: String) -> String {
    let mut count = 1;

    let mut compressed_string = "".to_string();

    let input_chars = input.chars();

    if input == "" {
        return compressed_string;
    }

    for (i, current) in input_chars.enumerate() {
        if i < input.len() - 1 && current == input.as_bytes()[i + 1] as char {
            count += 1;
        } else {
            if count > 2 {
                compressed_string.push_str(&format!("{}{}", current, count)[..]);
            } else {
                compressed_string.push_str(&current.to_string().repeat(count)[..]);
            }

            count = 1;
        }
    }

    compressed_string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aaabbc() {
        assert_eq!(compress("aaabbc".to_string()), "a3bbc")
    }

    #[test]
    fn aaaabbbccc() {
        assert_eq!(compress("aaaabbbccc".to_string()), "a4b3c3")
    }

    #[test]
    fn aaabaaa() {
        assert_eq!(compress("aaabaaa".to_string()), "a3ba3")
    }

    #[test]
    fn empty_string() {
        assert_eq!(compress("".to_string()), "")
    }

    #[test]
    fn abcd() {
        assert_eq!(compress("abcd".to_string()), "abcd")
    }

    #[test]
    fn aaaaa() {
        assert_eq!(compress("aaaaa".to_string()), "a5")
    }

    #[test]
    fn yyyyaaaabbbaaa() {
        assert_eq!(compress("yyyyaaaabbbaaa".to_string()), "y4a4b3a3")
    }
}
