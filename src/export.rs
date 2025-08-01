//! Audio file export functionality (WAV and MP3)

use std::path::Path;
use hound::{WavSpec, WavWriter, SampleFormat};
#[cfg(feature = "mp3")]
use lame::Lame;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::{SAMPLE_RATE, audio::{Oscillator, WaveForm}, music::Melody, error::Result};

/// Main generator for creating and exporting jingle audio
pub struct JingleGenerator {
    sample_rate: u32,
    rng: StdRng,
    current_seed: Option<u64>,
}

impl JingleGenerator {
    /// Create a new jingle generator with random seed
    pub fn new() -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            rng: StdRng::from_entropy(),
            current_seed: None,
        }
    }
    
    /// Create a new jingle generator with a specific seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            rng: StdRng::seed_from_u64(seed),
            current_seed: Some(seed),
        }
    }

    /// Create a reproducible generator from a string seed
    pub fn with_string_seed(seed: &str) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let numeric_seed = hasher.finish();
        
        Self::with_seed(numeric_seed)
    }

    /// Reset the RNG to the original seed (if available)
    pub fn reset_seed(&mut self) -> Result<()> {
        if let Some(seed) = self.current_seed {
            self.rng = StdRng::seed_from_u64(seed);
            Ok(())
        } else {
            Err(crate::error::JingleError::RandomError("No seed available to reset".to_string()))
        }
    }

    /// Set a new seed for the generator
    pub fn set_seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
        self.current_seed = Some(seed);
    }

    /// Get the current seed (if available)
    pub fn get_current_seed(&self) -> Option<u64> {
        self.current_seed
    }

    /// Create a new generator with the same seed but independent state
    pub fn fork(&self) -> Result<Self> {
        if let Some(seed) = self.current_seed {
            Ok(Self::with_seed(seed))
        } else {
            Err(crate::error::JingleError::RandomError("Cannot fork generator without seed".to_string()))
        }
    }

    /// Create a new generator with a derived seed for consistent but different randomness
    pub fn derive_seed(&self, variation: u64) -> Result<Self> {
        if let Some(seed) = self.current_seed {
            Ok(Self::with_seed(seed.wrapping_add(variation)))
        } else {
            Err(crate::error::JingleError::RandomError("Cannot derive seed without original seed".to_string()))
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
    
    /// Export audio samples to a file, detecting format from extension
    pub fn export_to_file<P: AsRef<Path>>(&self, samples: &[f32], path: P) -> Result<()> {
        let path_ref = path.as_ref();
        match path_ref.extension().and_then(|s| s.to_str()) {
            Some("wav") => self.export_to_wav(samples, path),
            #[cfg(feature = "mp3")]
            Some("mp3") => self.export_to_mp3(samples, path, 192), // Default to 192 kbps
            #[cfg(not(feature = "mp3"))]
            Some("mp3") => Err(crate::error::JingleError::Mp3Error("MP3 support not enabled. Compile with --features mp3".to_string())),
            _ => self.export_to_wav(samples, path), // Default to WAV
        }
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
    
    /// Export audio samples to an MP3 file
    #[cfg(feature = "mp3")]
    pub fn export_to_mp3<P: AsRef<Path>>(&self, samples: &[f32], path: P, bitrate: u32) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        // Initialize LAME encoder
        let mut lame = Lame::new()
            .ok_or_else(|| crate::error::JingleError::Mp3Error("Failed to create LAME encoder".to_string()))?;
        
        lame.set_channels(1)?;
        lame.set_sample_rate(self.sample_rate)?;
        lame.set_kilobitrate(bitrate as i32)?;
        lame.set_quality(0)?; // 0 = highest quality
        lame.init_params()?;
        
        // Convert f32 samples to i16 for LAME encoder
        let i16_samples: Vec<i16> = samples.iter()
            .map(|&sample| (sample * i16::MAX as f32) as i16)
            .collect();
        
        // Prepare output buffer (estimate size)
        let mut mp3_buffer = vec![0u8; i16_samples.len() * 5 / 4 + 7200];
        
        // Encode to MP3 (mono, so use same channel for left and right)
        let bytes_written = lame.encode(&i16_samples, &i16_samples, &mut mp3_buffer)?;
        mp3_buffer.truncate(bytes_written);
        
        // Write to file
        let mut file = File::create(path)?;
        file.write_all(&mp3_buffer)?;
        
        Ok(())
    }
    
    /// Export audio samples to MP3 with configurable bitrate
    #[cfg(feature = "mp3")]
    pub fn export_to_mp3_with_bitrate<P: AsRef<Path>>(&self, samples: &[f32], path: P, bitrate: u32) -> Result<()> {
        self.export_to_mp3(samples, path, bitrate)
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
    
    /// Get a random variation factor for parameters (0.6 to 1.4 range for more dramatic variation)
    pub fn random_variation(&mut self) -> f32 {
        self.rng.gen_range(0.6..=1.4)
    }
    
    /// Get a random pitch offset in semitones (-4 to +4 for wider pitch range)
    pub fn random_pitch_offset(&mut self) -> f32 {
        self.rng.gen_range(-4.0..=4.0)
    }
    
    /// Get a random rhythm variation factor (0.5 to 2.0 range)
    pub fn random_rhythm_variation(&mut self) -> f32 {
        self.rng.gen_range(0.5..=2.0)
    }
    
    /// Get a random note count variation for melodies (±1-2 notes)
    pub fn random_note_count_variation(&mut self, base_count: usize) -> usize {
        let variation = self.rng.gen_range(-2..=2);
        ((base_count as i32 + variation).max(2).min(10)) as usize
    }
    
    /// Choose a random scale from available options
    pub fn random_scale(&mut self) -> crate::music::Scale {
        let scales = [
            crate::music::Scale::Major,
            crate::music::Scale::Minor,
            crate::music::Scale::Pentatonic,
            crate::music::Scale::Chromatic,
        ];
        scales[self.rng.gen_range(0..scales.len())]
    }
    
    /// Choose a random melody pattern
    pub fn random_melody_pattern(&mut self) -> crate::music::MelodyPattern {
        let patterns = [
            crate::music::MelodyPattern::Ascending,
            crate::music::MelodyPattern::Descending,
            crate::music::MelodyPattern::Arpeggio,
            crate::music::MelodyPattern::ScaleRun,
            crate::music::MelodyPattern::Random,
        ];
        patterns[self.rng.gen_range(0..patterns.len())]
    }
    
    /// Generate a random boolean with given probability
    pub fn random_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }
    
    /// Generate a random integer in a range
    pub fn random_range(&mut self, range: std::ops::Range<i32>) -> i32 {
        self.rng.gen_range(range)
    }
    
    /// Generate a random float in a range
    pub fn random_float_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
        self.rng.gen_range(range)
    }
    
    /// Choose a random waveform for variety
    pub fn random_waveform(&mut self) -> WaveForm {
        let waveforms = [
            WaveForm::Sine,
            WaveForm::Triangle,
            WaveForm::Sawtooth,
            WaveForm::Square,
        ];
        waveforms[self.rng.gen_range(0..waveforms.len())]
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

    #[test]
    fn test_seeded_generation() {
        let mut gen1 = JingleGenerator::with_seed(12345);
        let mut gen2 = JingleGenerator::with_seed(12345);
        
        // Same seed should produce same results
        assert_eq!(gen1.random_variation(), gen2.random_variation());
        assert_eq!(gen1.random_pitch_offset(), gen2.random_pitch_offset());
    }

    #[test]
    fn test_string_seed() {
        let mut gen1 = JingleGenerator::with_string_seed("test");
        let mut gen2 = JingleGenerator::with_string_seed("test");
        
        // Same string seed should produce same results
        assert_eq!(gen1.random_variation(), gen2.random_variation());
    }

    #[test]
    fn test_seed_reset() {
        let mut generator = JingleGenerator::with_seed(12345);
        let first_value = generator.random_variation();
        generator.random_variation(); // Advance state
        
        // Reset should go back to beginning
        generator.reset_seed().unwrap();
        assert_eq!(generator.random_variation(), first_value);
    }

    #[test]
    fn test_fork_generator() {
        let mut original = JingleGenerator::with_seed(12345);
        let first_value = original.random_variation();
        
        // Fork should create independent copy with same seed
        let mut forked = original.fork().unwrap();
        assert_eq!(forked.random_variation(), first_value);
    }

    #[test]
    fn test_derive_seed() {
        let original = JingleGenerator::with_seed(12345);
        let derived = original.derive_seed(100).unwrap();
        
        // Derived generator should have different but consistent seed
        assert_eq!(derived.get_current_seed(), Some(12345u64.wrapping_add(100)));
    }

    #[test]
    fn test_no_seed_operations() {
        let generator = JingleGenerator::new();
        
        // Operations requiring seed should fail gracefully
        assert!(generator.fork().is_err());
        assert!(generator.derive_seed(100).is_err());
    }
}