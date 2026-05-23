use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct Smog;

impl ReadabilityMetric for Smog {
    fn name(&self) -> &str {
        "Smog"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        let TextMetrics {
            polysyllabic_count,
            sentence_count,
            ..
        } = text_metrics;

        let score =
            1.0430 / ((*polysyllabic_count as f64 * 30.0 / *sentence_count as f64).sqrt()) + 3.1291;

        score
    }
}
