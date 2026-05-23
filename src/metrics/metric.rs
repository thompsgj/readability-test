use crate::metrics::automated_readability_index::AutomatedReadabilityIndex;
use crate::metrics::coleman_liau::ColemanLiau;
use crate::metrics::flesch_kincaid::FleschKincaid;
use crate::metrics::flesch_reading_ease::FleschReadingEase;
use crate::metrics::gunning_fog::GunningFog;
use crate::metrics::lix::Lix;
use crate::metrics::rix::Rix;
use crate::metrics::smog::Smog;

use crate::metrics::readability_metric::ReadabilityMetric;
use crate::text_processing::text_metrics::TextMetrics;

pub enum Metric {
    AutomatedReadabilityIndex,
    ColemanLiau,
    FleschKincaid,
    FleschReadingEase,
    GunningFog,
    Lix,
    Rix,
    Smog,
}

impl Metric {
    pub fn calculate(&self, text: &String) -> f64 {
        let text_metrics: TextMetrics = TextMetrics::from(text);
        
        match self {
            Metric::AutomatedReadabilityIndex => AutomatedReadabilityIndex.calculate(&text_metrics),
            Metric::ColemanLiau => ColemanLiau.calculate(&text_metrics),
            Metric::FleschKincaid => FleschKincaid.calculate(&text_metrics),
            Metric::FleschReadingEase => FleschReadingEase.calculate(&text_metrics),
            Metric::GunningFog => GunningFog.calculate(&text_metrics),
            Metric::Lix => Lix.calculate(&text_metrics),
            Metric::Rix => Rix.calculate(&text_metrics),
            Metric::Smog => Smog.calculate(&text_metrics),
        }
    }
}
