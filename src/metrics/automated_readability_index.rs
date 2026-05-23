use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct AutomatedReadabilityIndex;

impl ReadabilityMetric for AutomatedReadabilityIndex {
    fn name(&self) -> &str {
        "Automated Readability Index"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            average_characters_per_word,
            average_words_per_sentence,
            ..
        } = text_metrics;

        let score = 4.71 * *average_characters_per_word as f64
            + 0.5 * *average_words_per_sentence as f64
            - 21.43;

        score
    }
}
