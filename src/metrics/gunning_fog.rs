use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub struct GunningFog;

impl ReadabilityMetric for GunningFog {
    fn name(&self) -> &str {
        "Gunning-Fog"
    }

    fn calculate(&self, text_metrics: &TextMetrics) -> f64 {
        // NOTE: Complex means words with 3 or more syllables that aren't proper nouns, compound words, or common suffixes (-ed, -ing)
        let TextMetrics {
            sentence_count,
            word_count,
            polysyllabic_count,
            ..
        } = text_metrics;
        let score = 0.4
            * ((*word_count as f64 / *sentence_count as f64)
                + (100.0 * (*polysyllabic_count as f64 / *word_count as f64)));

        score
    }
}
