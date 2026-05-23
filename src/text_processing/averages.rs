#[derive(PartialEq)]
pub enum WordUnit {
    Word,
    Sentence,
}

fn split_text<'a>(text: &'a str, word_unit: &WordUnit) -> Vec<&'a str> {
    if word_unit == &WordUnit::Word {
        text.split_whitespace().collect()
    } else if word_unit == &WordUnit::Sentence {
        text.split(['.', '!', '?']).collect()
    } else {
        Vec::<&str>::new()
    }
}

fn count_text_units(split_text: &Vec<&str>, word_unit: &WordUnit) -> Vec<usize> {
    let mut counts = Vec::<usize>::new();
    for text in split_text {
        if word_unit == &WordUnit::Word {
            counts.push(text.chars().count());
        } else if word_unit == &WordUnit::Sentence {
            counts.push(text.split_whitespace().count());
        } else {
            continue;
        }
    }
    counts
}

fn calculate_average(numerator: &usize, denominator: &usize) -> usize {
    numerator / denominator
}

pub fn calculate_average_of_word_unit(text: &str, word_unit: WordUnit) -> usize {
    let split_text: Vec<&str> = split_text(&text, &word_unit);
    let counts = count_text_units(&split_text, &word_unit);
    calculate_average(&counts.iter().sum(), &counts.len())
}
