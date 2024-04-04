use std::borrow::Cow;

pub fn ensure_suffix<'a>(input: &'a str, suffix: &'a str) -> Cow<'a, str> {
    if input.ends_with(suffix) {
        // No need to modify the string, so return a borrowed reference
        Cow::Borrowed(input)
    } else {
        // Need to modify the string, so clone it and append the suffix
        Cow::Owned(format!("{}{}", input, suffix))
    }
}
