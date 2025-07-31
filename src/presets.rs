//! Pre-built jingle presets for common notification types

use crate::{
    audio::WaveForm,
    music::{Note, Scale, Melody, MelodyPattern, Chord, ChordProgression},
    export::JingleGenerator,
    SAMPLE_RATE,
};

impl JingleGenerator {
    /// Create a pleasant notification sound using C major pentatonic
    pub fn create_notification_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        let melody = Melody::from_scale(
            Scale::Pentatonic, 
            Note::C, 
            5, 
            MelodyPattern::Arpeggio, 
            0.15
        );
        
        self.generate_melody_samples(&melody, 5, waveform)
    }
    
    /// Create an uplifting success sound using ascending C major
    pub fn create_success_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        let melody = Melody::from_scale(
            Scale::Major,
            Note::C,
            4,
            MelodyPattern::Ascending,
            0.2
        );
        
        self.generate_melody_samples(&melody, 4, waveform)
    }
    
    /// Create an attention-grabbing alert using repeated high notes
    pub fn create_alert_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        let mut samples = Vec::new();
        
        // Two short beeps
        for _ in 0..2 {
            let beep_samples = self.generate_tone(Note::G.frequency(6), 0.1, waveform);
            samples.extend(beep_samples);
            
            // Small gap between beeps
            let silence_samples = (SAMPLE_RATE as f32 * 0.05) as usize;
            samples.extend(vec![0.0; silence_samples]);
        }
        
        samples
    }
    
    /// Create an error/warning sound with descending minor pattern
    pub fn create_error_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        let melody = Melody::from_scale(
            Scale::Minor,
            Note::D,
            5,
            MelodyPattern::Descending,
            0.25
        );
        
        self.generate_melody_samples(&melody, 5, waveform)
    }
    
    /// Create a startup chime with chord progression
    pub fn create_startup_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        let chord_progression = ChordProgression::Pop.get_chords(Note::C);
        let mut chord_samples = Vec::new();
        
        // Play only the first two chords (I-V) for a nice startup sound
        for chord in chord_progression.iter().take(2) {
            let chord_melody = Melody::from_chord(chord.clone(), 4, MelodyPattern::Arpeggio, 0.3);
            let samples = self.generate_melody_samples(&chord_melody, 4, waveform);
            chord_samples.extend(samples);
        }
        
        chord_samples
    }
    
    /// Create a shutdown sound with gentle descending pattern
    pub fn create_shutdown_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        let melody = Melody::from_scale(
            Scale::Pentatonic,
            Note::G,
            4,
            MelodyPattern::Descending,
            0.4
        );
        
        self.generate_melody_samples(&melody, 4, waveform)
    }
    
    /// Create a message received sound - short and pleasant
    pub fn create_message_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        // Two ascending notes
        let mut samples = Vec::new();
        
        let note1_samples = self.generate_tone(Note::C.frequency(5), 0.1, waveform);
        let note2_samples = self.generate_tone(Note::E.frequency(5), 0.15, waveform);
        
        samples.extend(note1_samples);
        samples.extend(note2_samples);
        samples
    }
    
    /// Create a completion/done sound with satisfying resolution
    pub fn create_completion_jingle(&self, waveform: WaveForm) -> Vec<f32> {
        // Perfect cadence: G -> C (V -> I)
        let mut samples = Vec::new();
        
        let g_chord = Chord::major(Note::G);
        let c_chord = Chord::major(Note::C);
        
        let g_melody = Melody::from_chord(g_chord, 4, MelodyPattern::Arpeggio, 0.2);
        let c_melody = Melody::from_chord(c_chord, 4, MelodyPattern::Arpeggio, 0.3);
        
        samples.extend(self.generate_melody_samples(&g_melody, 4, waveform));
        samples.extend(self.generate_melody_samples(&c_melody, 4, waveform));
        
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
    pub fn generate(&self, generator: &JingleGenerator, waveform: WaveForm) -> Vec<f32> {
        match self {
            JinglePreset::Notification => generator.create_notification_jingle(waveform),
            JinglePreset::Success => generator.create_success_jingle(waveform),
            JinglePreset::Alert => generator.create_alert_jingle(waveform),
            JinglePreset::Error => generator.create_error_jingle(waveform),
            JinglePreset::Startup => generator.create_startup_jingle(waveform),
            JinglePreset::Shutdown => generator.create_shutdown_jingle(waveform),
            JinglePreset::Message => generator.create_message_jingle(waveform),
            JinglePreset::Completion => generator.create_completion_jingle(waveform),
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
        let generator = JingleGenerator::new();
        let samples = generator.create_notification_jingle(WaveForm::Sine);
        assert!(!samples.is_empty());
    }

    #[test]
    fn test_all_presets() {
        let generator = JingleGenerator::new();
        
        for preset in JinglePreset::all() {
            let samples = preset.generate(&generator, WaveForm::Sine);
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