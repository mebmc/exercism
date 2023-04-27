use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    return String::from(input).graphemes(true).rev().collect();
}
