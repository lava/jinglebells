//! Pre-built jingle presets for common notification types

use crate::{
    audio::WaveForm,
    music::{Note, Scale, Melody, MelodyPattern, Chord, ChordProgression},
    export::JingleGenerator,
    SAMPLE_RATE,
};

impl JingleGenerator {
    /// Create a pleasant notification sound with varied scales and patterns
    pub fn create_notification_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let base_duration = duration.unwrap_or(0.15) * self.random_variation();
        let note_count = self.random_note_count_variation(4);
        let note_duration = (base_duration / note_count as f32) * self.random_rhythm_variation();
        
        let base_freq = base_frequency.unwrap_or(Note::C.frequency(5));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_freq);
        
        // Randomly choose between pleasant scales and patterns
        let scale = if self.random_bool(0.7) { Scale::Pentatonic } else { Scale::Major };
        let pattern = if self.random_bool(0.6) { MelodyPattern::Arpeggio } else { self.random_melody_pattern() };
        let used_waveform = if self.random_bool(0.3) { self.random_waveform() } else { waveform };
        
        let melody = Melody::from_scale(
            scale, 
            root_note, 
            5, 
            pattern, 
            note_duration
        );
        
        self.generate_melody_samples(&melody, 5, used_waveform)
    }
    
    /// Create an uplifting success sound with varied upward patterns
    pub fn create_success_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let base_duration = duration.unwrap_or(0.8) * self.random_variation();
        let note_count = self.random_note_count_variation(5);
        let note_duration = (base_duration / note_count as f32) * self.random_rhythm_variation();
        
        let base_freq = base_frequency.unwrap_or(Note::C.frequency(4));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_freq);
        
        // Prefer uplifting scales and ascending patterns
        let scale = if self.random_bool(0.6) { Scale::Major } else { Scale::Pentatonic };
        let pattern = if self.random_bool(0.7) { MelodyPattern::Ascending } else { MelodyPattern::Arpeggio };
        let used_waveform = if self.random_bool(0.4) { self.random_waveform() } else { waveform };
        
        let melody = Melody::from_scale(
            scale,
            root_note,
            4,
            pattern,
            note_duration
        );
        
        self.generate_melody_samples(&melody, 4, used_waveform)
    }
    
    /// Create an attention-grabbing alert with varied patterns and intensity
    pub fn create_alert_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let mut samples = Vec::new();
        
        // Calculate beep duration based on total duration
        let total_duration = duration.unwrap_or(0.25) * self.random_variation();
        let beep_count = self.random_note_count_variation(2).max(2).min(4);
        let beep_duration = (total_duration / (beep_count as f32 * 1.5)) * self.random_rhythm_variation();
        let gap_duration = beep_duration * self.random_float_range(0.3..=0.8);
        
        let base_freq = base_frequency.unwrap_or(Note::G.frequency(6));
        let pitch_offset = self.random_pitch_offset();
        let frequency = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        
        // Use harsher waveforms for alerts with some variation
        let used_waveform = if self.random_bool(0.3) { 
            match self.random_range(0..2) {
                0 => WaveForm::Square,
                _ => WaveForm::Sawtooth,
            }
        } else { 
            waveform 
        };
        
        // Variable number of beeps with slight frequency variations
        for i in 0..beep_count {
            let freq_variation = if i > 0 { self.random_float_range(0.95..=1.05) } else { 1.0 };
            let varied_freq = frequency * freq_variation;
            let beep_samples = self.generate_tone(varied_freq, beep_duration, used_waveform);
            samples.extend(beep_samples);
            
            // Add gap between beeps (except after the last one)
            if i < beep_count - 1 {
                let silence_samples = (SAMPLE_RATE as f32 * gap_duration) as usize;
                samples.extend(vec![0.0; silence_samples]);
            }
        }
        
        samples
    }
    
    /// Create an error/warning sound with varied minor patterns and dissonance
    pub fn create_error_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let base_duration = duration.unwrap_or(1.25) * self.random_variation();
        let note_count = self.random_note_count_variation(5);
        let note_duration = (base_duration / note_count as f32) * self.random_rhythm_variation();
        
        let base_freq = base_frequency.unwrap_or(Note::D.frequency(5));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_freq);
        
        // Prefer more dissonant/unsettling combinations
        let scale = if self.random_bool(0.7) { Scale::Minor } else { Scale::Chromatic };
        let pattern = if self.random_bool(0.5) { MelodyPattern::Descending } else { MelodyPattern::Random };
        let used_waveform = if self.random_bool(0.4) { 
            // Prefer harsher waveforms for errors
            match self.random_range(0..3) {
                0 => WaveForm::Sawtooth,
                1 => WaveForm::Square,
                _ => waveform,
            }
        } else { 
            waveform 
        };
        
        let melody = Melody::from_scale(
            scale,
            root_note,
            5,
            pattern,
            note_duration
        );
        
        self.generate_melody_samples(&melody, 5, used_waveform)
    }
    
    /// Create a startup chime with varied chord progressions and patterns
    pub fn create_startup_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let base_duration = duration.unwrap_or(0.6) * self.random_variation();
        let chord_count = self.random_note_count_variation(2).max(2).min(4);
        let chord_duration = (base_duration / chord_count as f32) * self.random_rhythm_variation();
        
        let base_freq = base_frequency.unwrap_or(Note::C.frequency(4));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_freq);
        
        // Vary chord progressions for different startup sounds
        let progression = match self.random_range(0..3) {
            0 => ChordProgression::Pop,
            1 => ChordProgression::Classical,
            _ => ChordProgression::Jazz,
        };
        let chord_progression = progression.get_chords(root_note);
        let mut chord_samples = Vec::new();
        
        let pattern = if self.random_bool(0.7) { MelodyPattern::Arpeggio } else { MelodyPattern::Ascending };
        let used_waveform = if self.random_bool(0.3) { self.random_waveform() } else { waveform };
        
        // Play a variable number of chords
        for chord in chord_progression.iter().take(chord_count) {
            let chord_melody = Melody::from_chord(chord.clone(), 4, pattern, chord_duration);
            let samples = self.generate_melody_samples(&chord_melody, 4, used_waveform);
            chord_samples.extend(samples);
        }
        
        chord_samples
    }
    
    /// Create a shutdown sound with varied gentle descending patterns
    pub fn create_shutdown_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let base_duration = duration.unwrap_or(1.6) * self.random_variation();
        let note_count = self.random_note_count_variation(4);
        let note_duration = (base_duration / note_count as f32) * self.random_rhythm_variation();
        
        let base_freq = base_frequency.unwrap_or(Note::G.frequency(4));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_freq);
        
        // Prefer gentle, calming scales and patterns
        let scale = if self.random_bool(0.6) { Scale::Pentatonic } else { Scale::Major };
        let pattern = if self.random_bool(0.8) { MelodyPattern::Descending } else { MelodyPattern::ScaleRun };
        let used_waveform = if self.random_bool(0.2) { 
            // Prefer softer waveforms for shutdown
            match self.random_range(0..2) {
                0 => WaveForm::Sine,
                _ => WaveForm::Triangle,
            }
        } else { 
            waveform 
        };
        
        let melody = Melody::from_scale(
            scale,
            root_note,
            4,
            pattern,
            note_duration
        );
        
        self.generate_melody_samples(&melody, 4, used_waveform)
    }
    
    /// Create a message received sound with varied short pleasant patterns
    pub fn create_message_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let mut samples = Vec::new();
        
        let total_duration = duration.unwrap_or(0.25) * self.random_variation();
        let note_count = self.random_note_count_variation(2).max(2).min(3);
        
        let base_freq = base_frequency.unwrap_or(Note::C.frequency(5));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_base_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_base_freq);
        
        let used_waveform = if self.random_bool(0.3) { self.random_waveform() } else { waveform };
        
        if note_count == 2 {
            // Traditional two-note pattern with variations
            let rhythm_var = self.random_rhythm_variation();
            let note1_duration = (total_duration * 0.4) * rhythm_var;
            let note2_duration = (total_duration * 0.6) * rhythm_var;
            
            // Vary the interval between notes
            let interval_semitones = match self.random_range(0..4) {
                0 => 4, // Major third
                1 => 5, // Perfect fourth
                2 => 7, // Perfect fifth
                _ => 3, // Minor third
            };
            let note2_freq = adjusted_base_freq * (2.0_f32).powf(interval_semitones as f32 / 12.0);
            
            let note1_samples = self.generate_tone(adjusted_base_freq, note1_duration, used_waveform);
            let note2_samples = self.generate_tone(note2_freq, note2_duration, used_waveform);
            
            samples.extend(note1_samples);
            samples.extend(note2_samples);
        } else {
            // Three-note arpeggio pattern
            let note_duration = total_duration / 3.0;
            let chord = Chord::major(root_note);
            let melody = Melody::from_chord(chord, 5, MelodyPattern::Arpeggio, note_duration);
            samples = self.generate_melody_samples(&melody, 5, used_waveform);
        }
        
        samples
    }
    
    /// Create a completion/done sound with varied satisfying resolutions
    pub fn create_completion_jingle(&mut self, waveform: WaveForm, duration: Option<f32>, base_frequency: Option<f32>) -> Vec<f32> {
        let mut samples = Vec::new();
        
        let base_duration = duration.unwrap_or(0.5) * self.random_variation();
        let chord_count = self.random_note_count_variation(2).max(2).min(3);
        let chord_duration = (base_duration / chord_count as f32) * self.random_rhythm_variation();
        
        let base_freq = base_frequency.unwrap_or(Note::C.frequency(4));
        let pitch_offset = self.random_pitch_offset();
        let adjusted_freq = base_freq * (2.0_f32).powf(pitch_offset / 12.0);
        let root_note = Note::from_frequency(adjusted_freq);
        
        let used_waveform = if self.random_bool(0.3) { self.random_waveform() } else { waveform };
        
        if chord_count == 2 {
            // Traditional V-I cadence with variations
            let fifth_note = Note::from_frequency(root_note.frequency(4) * 1.5); // Perfect fifth ratio
            
            let first_chord = if self.random_bool(0.7) { 
                Chord::major(fifth_note) 
            } else { 
                Chord::dominant7(fifth_note) 
            };
            let final_chord = Chord::major(root_note);
            
            let pattern = if self.random_bool(0.8) { MelodyPattern::Arpeggio } else { MelodyPattern::Ascending };
            
            let first_melody = Melody::from_chord(first_chord, 4, pattern, chord_duration * 0.8);
            let final_melody = Melody::from_chord(final_chord, 4, pattern, chord_duration * 1.2);
            
            samples.extend(self.generate_melody_samples(&first_melody, 4, used_waveform));
            samples.extend(self.generate_melody_samples(&final_melody, 4, used_waveform));
        } else {
            // Three-chord resolution (vi-V-I or ii-V-I)
            let progression = if self.random_bool(0.5) { ChordProgression::Jazz } else { ChordProgression::Classical };
            let chords = progression.get_chords(root_note);
            
            let pattern = if self.random_bool(0.7) { MelodyPattern::Arpeggio } else { MelodyPattern::Ascending };
            
            for chord in chords.iter().take(3) {
                let melody = Melody::from_chord(chord.clone(), 4, pattern, chord_duration);
                let chord_samples = self.generate_melody_samples(&melody, 4, used_waveform);
                samples.extend(chord_samples);
            }
        }
        
        samples
    }
}

/// Preset jingle types available in the library
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JinglePreset {
    Notification,
    Success,
    Alert,
    Error,
    Startup,
    Shutdown,
    Message,
    Completion,
}

impl JinglePreset {
    /// Generate samples for this preset using the specified waveform
    pub fn generate(&self, generator: &mut JingleGenerator, waveform: WaveForm) -> Vec<f32> {
        self.generate_with_params(generator, waveform, None, None)
    }
    
    /// Generate samples for this preset with optional duration and frequency parameters
    pub fn generate_with_params(
        &self, 
        generator: &mut JingleGenerator, 
        waveform: WaveForm,
        duration: Option<f32>,
        frequency: Option<f32>
    ) -> Vec<f32> {
        match self {
            JinglePreset::Notification => generator.create_notification_jingle(waveform, duration, frequency),
            JinglePreset::Success => generator.create_success_jingle(waveform, duration, frequency),
            JinglePreset::Alert => generator.create_alert_jingle(waveform, duration, frequency),
            JinglePreset::Error => generator.create_error_jingle(waveform, duration, frequency),
            JinglePreset::Startup => generator.create_startup_jingle(waveform, duration, frequency),
            JinglePreset::Shutdown => generator.create_shutdown_jingle(waveform, duration, frequency),
            JinglePreset::Message => generator.create_message_jingle(waveform, duration, frequency),
            JinglePreset::Completion => generator.create_completion_jingle(waveform, duration, frequency),
        }
    }
    
    /// Get all available presets
    pub fn all() -> Vec<JinglePreset> {
        vec![
            JinglePreset::Notification,
            JinglePreset::Success,
            JinglePreset::Alert,
            JinglePreset::Error,
            JinglePreset::Startup,
            JinglePreset::Shutdown,
            JinglePreset::Message,
            JinglePreset::Completion,
        ]
    }
    
    /// Get the name of this preset as a string
    pub fn name(&self) -> &'static str {
        match self {
            JinglePreset::Notification => "notification",
            JinglePreset::Success => "success",
            JinglePreset::Alert => "alert",
            JinglePreset::Error => "error",
            JinglePreset::Startup => "startup",
            JinglePreset::Shutdown => "shutdown",
            JinglePreset::Message => "message",
            JinglePreset::Completion => "completion",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_jingle() {
        let mut generator = JingleGenerator::new();
        let samples = generator.create_notification_jingle(WaveForm::Sine, None, None);
        assert!(!samples.is_empty());
    }

    #[test]
    fn test_all_presets() {
        let mut generator = JingleGenerator::new();
        
        for preset in JinglePreset::all() {
            let samples = preset.generate(&mut generator, WaveForm::Sine);
            assert!(!samples.is_empty(), "Preset {} should generate samples", preset.name());
        }
    }

    #[test]
    fn test_preset_names() {
        assert_eq!(JinglePreset::Notification.name(), "notification");
        assert_eq!(JinglePreset::Success.name(), "success");
        assert_eq!(JinglePreset::Alert.name(), "alert");
    }
}