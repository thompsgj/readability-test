use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct FleschReadingEase;

impl ReadabilityMetric for FleschReadingEase {
    fn name(&self) -> &str {
        "Flesch Reading Ease"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            sentence_count,
            word_count,
            syllable_count,
            ..
        } = text_metrics;

        let score = 206.835
            - 1.015 * (*word_count as f64 / *sentence_count as f64)
            - 84.6 * (*syllable_count as f64 / *word_count as f64);

        score
    }
}
