use crate::text_processing::text_metrics::TextMetrics;

pub trait ReadabilityMetric {
    fn name(&self) -> &str;
    fn calculate(&self, text_metrics: &TextMetrics) -> f64;
}
