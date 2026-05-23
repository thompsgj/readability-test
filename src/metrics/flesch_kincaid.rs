use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct FleschKincaid;

impl ReadabilityMetric for FleschKincaid {
    fn name(&self) -> &str {
        "Flesch-Kincaid"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            sentence_count,
            word_count,
            syllable_count,
            ..
        } = text_metrics;

        let score = 0.39 * (*word_count as f64 / *sentence_count as f64)
            + 11.8 * (*syllable_count as f64 / *word_count as f64)
            - 15.59;

        score
    }
}
