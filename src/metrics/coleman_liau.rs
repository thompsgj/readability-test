use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct ColemanLiau;

impl ReadabilityMetric for ColemanLiau {
    fn name(&self) -> &str {
        "Coleman Liau"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            letter_count,
            word_count,
            sentence_count,
            ..
        } = text_metrics;

        let score = (0.0588 * (*letter_count as f64 / *word_count as f64 * 100.0))
            - (0.296 * (*sentence_count as f64 / *word_count as f64 * 100.0))
            - 15.8;

        score
    }
}
