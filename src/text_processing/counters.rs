use syllarust::count_syllables;

pub fn get_word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn get_sentence_count(text: &str) -> usize {
    text.matches(['.', '!', '?']).count()
}

pub fn get_letter_count(text: &str) -> usize {
    text.chars().filter(|c| c.is_alphabetic()).count()
}

pub fn get_syllable_count(text: &str) -> usize {
    let total_syllables: usize = text
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|word| !word.is_empty())
        .map(count_syllables)
        .sum();
    total_syllables
}

pub fn get_long_word_count(text: &str) -> usize {
    let words = text.split_whitespace();
    let mut long_word_count = 0;
    for word in words {
        if word.chars().count() >= 7 {
            long_word_count += 1;
        }
    }

    long_word_count
}

pub fn get_polysyllabic_count(text: &str) -> usize {
    let words = text.split_whitespace();
    let mut polysyllabic_count = 0;
    for word in words {
        let syllable_count = count_syllables(word);
        if syllable_count >= 3 {
            polysyllabic_count += 1;
        }
    }

    polysyllabic_count
}
