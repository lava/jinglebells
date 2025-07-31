//! WAV file export functionality

use std::path::Path;
use hound::{WavSpec, WavWriter, SampleFormat};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::{SAMPLE_RATE, audio::{Oscillator, WaveForm}, music::Melody, error::Result};

/// Main generator for creating and exporting jingle audio
pub struct JingleGenerator {
    sample_rate: u32,
    rng: StdRng,
}

impl JingleGenerator {
    /// Create a new jingle generator with random seed
    pub fn new() -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            rng: StdRng::from_entropy(),
        }
    }
    
    /// Create a new jingle generator with a specific seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            rng: StdRng::seed_from_u64(seed),
        }
    }
    
    /// Generate audio samples from a melody
    pub fn generate_melody_samples(&self, melody: &Melody, octave: i32, waveform: WaveForm) -> Vec<f32> {
        let mut all_samples = Vec::new();
        
        for (note, duration) in &melody.notes {
            let frequency = note.frequency(octave);
            let oscillator = Oscillator::new(frequency, waveform, *duration);
            let samples: Vec<f32> = oscillator.collect();
            all_samples.extend(samples);
        }
        
        all_samples
    }
    
    /// Export audio samples to a WAV file
    pub fn export_to_wav<P: AsRef<Path>>(&self, samples: &[f32], path: P) -> Result<()> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        
        let mut writer = WavWriter::create(path, spec)?;
        
        for &sample in samples {
            // Convert f32 sample (-1.0 to 1.0) to i16
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            writer.write_sample(sample_i16)?;
        }
        
        writer.finalize()?;
        Ok(())
    }
    
    /// Generate a single tone with specified parameters
    pub fn generate_tone(&self, frequency: f32, duration: f32, waveform: WaveForm) -> Vec<f32> {
        let oscillator = Oscillator::new(frequency, waveform, duration);
        oscillator.collect()
    }
    
    /// Combine multiple sample arrays with optional gaps
    pub fn combine_samples(&self, sample_arrays: &[Vec<f32>], gap_duration: f32) -> Vec<f32> {
        let mut combined = Vec::new();
        let gap_samples = (self.sample_rate as f32 * gap_duration) as usize;
        
        for (i, samples) in sample_arrays.iter().enumerate() {
            combined.extend(samples);
            
            // Add gap between samples (except after the last one)
            if i < sample_arrays.len() - 1 && gap_duration > 0.0 {
                combined.extend(vec![0.0; gap_samples]);
            }
        }
        
        combined
    }
    
    /// Get a random variation factor for parameters (0.8 to 1.2 range)
    pub fn random_variation(&mut self) -> f32 {
        self.rng.gen_range(0.8..=1.2)
    }
    
    /// Get a random pitch offset in semitones (-2 to +2)
    pub fn random_pitch_offset(&mut self) -> f32 {
        self.rng.gen_range(-2.0..=2.0)
    }
}

impl Default for JingleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::{Note, Scale, MelodyPattern, Melody};

    #[test]
    fn test_generator_creation() {
        let generator = JingleGenerator::new();
        assert_eq!(generator.sample_rate, SAMPLE_RATE);
    }

    #[test]
    fn test_tone_generation() {
        let generator = JingleGenerator::new();
        let samples = generator.generate_tone(440.0, 0.1, WaveForm::Sine);
        assert!(!samples.is_empty());
        assert_eq!(samples.len(), (SAMPLE_RATE as f32 * 0.1) as usize);
    }

    #[test]
    fn test_melody_sample_generation() {
        let generator = JingleGenerator::new();
        let melody = Melody::from_scale(Scale::Major, Note::C, 4, MelodyPattern::Ascending, 0.1);
        let samples = generator.generate_melody_samples(&melody, 4, WaveForm::Sine);
        assert!(!samples.is_empty());
    }

    #[test]
    fn test_sample_combination() {
        let generator = JingleGenerator::new();
        let samples1 = vec![0.1, 0.2, 0.3];
        let samples2 = vec![0.4, 0.5, 0.6];
        let combined = generator.combine_samples(&[samples1, samples2], 0.0);
        assert_eq!(combined.len(), 6);
        assert_eq!(combined[0], 0.1);
        assert_eq!(combined[3], 0.4);
    }
}