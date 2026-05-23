use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct Rix;

impl ReadabilityMetric for Rix {
    fn name(&self) -> &str {
        "Rix"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            long_word_count,
            sentence_count,
            ..
        } = text_metrics;

        let score: f64 = *long_word_count as f64 / *sentence_count as f64;

        score as f64
    }
}
