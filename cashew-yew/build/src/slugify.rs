use deunicode::deunicode;
use regex::Regex;

/// Port of scripts/slugify.ts — generate URL-safe heading slugs.
///
/// Algorithm:
/// 1. Transliterate unicode to ASCII (remove diacritics)
/// 2. Remove control characters (0x00-0x1F)
/// 3. Replace special characters with hyphens
/// 4. Remove consecutive hyphens
/// 5. Trim leading/trailing hyphens
/// 6. Prefix numeric starts with underscore
/// 7. Lowercase
pub fn slugify(input: &str) -> String {
    let ascii = deunicode(input);

    // Remove control characters
    let re_control = Regex::new(r"[\x00-\x1F]").unwrap();
    let result = re_control.replace_all(&ascii, "");

    // Replace special characters with hyphens
    let re_special = Regex::new(r#"[\s~`!@#$%^&*()\-_+=\[\]{}|\\;:"'<>,.?/]+"#).unwrap();
    let result = re_special.replace_all(&result, "-");

    // Remove consecutive hyphens
    let re_consecutive = Regex::new(r"-{2,}").unwrap();
    let result = re_consecutive.replace_all(&result, "-");

    // Trim leading and trailing hyphens
    let result = result.trim_matches('-');

    // Ensure it doesn't start with a number
    let re_leading_digit = Regex::new(r"^(\d)").unwrap();
    let result = re_leading_digit.replace(&result, "_$1");

    result.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_special_chars() {
        assert_eq!(slugify("What's the #1 Thing?"), "whats-the-_1-thing");
    }

    #[test]
    fn test_diacritics() {
        assert_eq!(slugify("Café résumé"), "cafe-resume");
    }

    #[test]
    fn test_leading_number() {
        assert_eq!(slugify("1st heading"), "_1st-heading");
    }

    #[test]
    fn test_empty() {
        assert_eq!(slugify(""), "");
    }
}
