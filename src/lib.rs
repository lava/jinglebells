//! # Jingle Maker
//! 
//! A Rust library for generating pleasant-sounding jingles and notification sounds
//! using custom oscillators and musical theory.
//!
//! ## Features
//! 
//! - Custom oscillator synthesis with multiple waveforms (sine, triangle, sawtooth, square)
//! - ADSR envelope control for natural-sounding audio
//! - Musical theory support with scales, chords, and progressions
//! - WAV file export functionality
//! - Preset jingle generators for common notification types
//!
//! ## Example
//! 
//! ```rust
//! use jinglemaker::{JingleGenerator, WaveForm};
//! 
//! let mut generator = JingleGenerator::new();
//! let samples = generator.create_notification_jingle(WaveForm::Sine, None, None);
//! generator.export_to_wav(&samples, "notification.wav").unwrap();
//! ```

pub mod audio;
pub mod music;
pub mod export;
pub mod presets;
pub mod error;
pub mod effects;

pub use audio::{WaveForm, ADSR, Oscillator, LayeredOscillator, OscillatorLayer};
pub use music::{Note, Scale, Chord, ChordProgression, Melody, MelodyPattern, RhythmPattern};
pub use export::JingleGenerator;
pub use presets::*;
pub use error::JingleError;
pub use effects::{DelayBuffer, Echo, Reverb, LowPassFilter, LowPass, AutomaticGainControl, AGC, normalize_samples, peak_normalize};

/// Standard sample rate used throughout the library
pub const SAMPLE_RATE: u32 = 44100;

/// A4 frequency reference for musical note calculations
pub const A4_FREQUENCY: f32 = 440.0;