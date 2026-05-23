use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct Lix;

impl ReadabilityMetric for Lix {
    fn name(&self) -> &str {
        "Lix"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            long_word_count,
            average_words_per_sentence,
            ..
        } = text_metrics;

        let score: f64 = *long_word_count as f64 / *average_words_per_sentence as f64;

        score as f64
    }
}
