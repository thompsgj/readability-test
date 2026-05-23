use crate::text_processing::averages::{WordUnit, calculate_average_of_word_unit};
use crate::text_processing::counters::{
    get_letter_count, get_long_word_count, get_polysyllabic_count, get_sentence_count,
    get_syllable_count, get_word_count,
};

pub struct TextMetrics {
    pub letter_count: usize,
    pub sentence_count: usize,
    pub word_count: usize,
    pub syllable_count: usize,
    pub average_words_per_sentence: usize,
    pub average_characters_per_word: usize,
    pub long_word_count: usize, // Lix/Rix = 7 or more letters
    pub polysyllabic_count: usize,
    // pub complex_word_count: usize, // Complex means words with 3 or more syllables that aren't proper nouns, compound words, or common suffixes (-ed, -ing)
}

impl From<&String> for TextMetrics {
    fn from(file_contents: &String) -> Self {
        let letter_count = get_letter_count(&file_contents);
        let sentence_count = get_sentence_count(&file_contents);
        let word_count = get_word_count(&file_contents);
        let syllable_count = get_syllable_count(&file_contents);
        let average_words_per_sentence =
            calculate_average_of_word_unit(&file_contents, WordUnit::Sentence);
        let average_characters_per_word =
            calculate_average_of_word_unit(&file_contents, WordUnit::Word);
        let long_word_count = get_long_word_count(&file_contents);
        let polysyllabic_count = get_polysyllabic_count(&file_contents);

        Self {
            letter_count,
            sentence_count,
            word_count,
            syllable_count,
            average_words_per_sentence,
            average_characters_per_word,
            long_word_count,
            polysyllabic_count,
        }
    }
}
