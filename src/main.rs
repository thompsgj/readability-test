use std::error::Error;
use std::fs;

pub mod metrics;
pub mod text_processing;

use crate::metrics::metric::Metric;

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file_contents = fs::read_to_string(file_path)?;

    Ok(file_contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open File
    let file_path: &str = "src/readings/chicago.txt";
    let file_contents = match read_file(&file_path) {
        Ok(file_contents) => file_contents,
        Err(_err) => {
            eprintln!("Failed to open {}", &file_path);
            return Ok(());
        }
    };

    let result = Metric::FleschKincaid.calculate(&file_contents);
    println!("Flesch Kincaid: {result}");

    let result = Metric::FleschKincaid.calculate(&file_contents);
    println!("Flesch Kincaid: {result}");

    let result2 = Metric::FleschReadingEase.calculate(&file_contents);
    println!("Flesch Reading Ease: {result2}");

    let result3 = Metric::AutomatedReadabilityIndex.calculate(&file_contents);
    println!("Automated Readability Index: {result3}");

    let result4 = Metric::Lix.calculate(&file_contents);
    println!("Lix: {:.2}", result4);

    let result5 = Metric::Rix.calculate(&file_contents);
    println!("Rix: {:.2}", result5);

    let result6 = Metric::Smog.calculate(&file_contents);
    println!("Smog: {result6}");

    let result7 = Metric::GunningFog.calculate(&file_contents);
    println!("Gunning Fog: {result7}");

    let result8 = Metric::ColemanLiau.calculate(&file_contents);
    println!("Coleman Liau: {result8}");

    Ok(())
}
