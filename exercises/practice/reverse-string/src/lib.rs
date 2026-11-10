#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    #[cfg(feature = "grapheme")]
    {
        input.graphemes(true).rev().collect()
    }
    #[cfg(not(feature = "grapheme"))]
    {
        input.chars().rev().collect()
    }
}
