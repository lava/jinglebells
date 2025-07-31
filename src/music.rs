//! Musical theory components for jingle generation

use crate::{A4_FREQUENCY};

/// Musical notes with semitone calculations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Note {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}

impl Note {
    /// Get the semitone offset from A4
    pub fn semitone_offset(&self) -> i32 {
        match self {
            Note::C => -9,
            Note::CSharp => -8,
            Note::D => -7,
            Note::DSharp => -6,
            Note::E => -5,
            Note::F => -4,
            Note::FSharp => -3,
            Note::G => -2,
            Note::GSharp => -1,
            Note::A => 0,
            Note::ASharp => 1,
            Note::B => 2,
        }
    }
    
    /// Calculate the frequency for this note in a given octave
    pub fn frequency(&self, octave: i32) -> f32 {
        let semitone_offset = self.semitone_offset() + (octave - 4) * 12;
        A4_FREQUENCY * 2.0_f32.powf(semitone_offset as f32 / 12.0)
    }
    
    /// Find the closest note to a given frequency
    pub fn from_frequency(frequency: f32) -> Note {
        // Calculate semitones from A4
        let semitones_from_a4 = 12.0 * (frequency / A4_FREQUENCY).log2();
        let rounded_semitones = semitones_from_a4.round() as i32;
        
        // Normalize to range 0-11 (within an octave)
        let note_index = ((rounded_semitones % 12) + 12) % 12;
        
        // Map to note based on distance from A
        match note_index {
            0 => Note::A,
            1 => Note::ASharp,
            2 => Note::B,
            3 => Note::C,
            4 => Note::CSharp,
            5 => Note::D,
            6 => Note::DSharp,
            7 => Note::E,
            8 => Note::F,
            9 => Note::FSharp,
            10 => Note::G,
            11 => Note::GSharp,
            _ => Note::A, // Should never happen due to modulo
        }
    }
}

/// Musical scales with interval definitions
#[derive(Clone, Debug, PartialEq)]
pub enum Scale {
    Major,
    Minor,
    Pentatonic,
    Chromatic,
}

impl Scale {
    /// Get the interval pattern for this scale
    pub fn intervals(&self) -> Vec<i32> {
        match self {
            Scale::Major => vec![0, 2, 4, 5, 7, 9, 11],
            Scale::Minor => vec![0, 2, 3, 5, 7, 8, 10],
            Scale::Pentatonic => vec![0, 2, 4, 7, 9],
            Scale::Chromatic => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        }
    }
    
    /// Get the notes for this scale starting from a root note
    pub fn notes(&self, root: Note) -> Vec<Note> {
        let root_semitone = root.semitone_offset();
        self.intervals()
            .iter()
            .map(|&interval| {
                let semitone = (root_semitone + interval + 120) % 12;
                match semitone {
                    -9 | 3 => Note::C,
                    -8 | 4 => Note::CSharp,
                    -7 | 5 => Note::D,
                    -6 | 6 => Note::DSharp,
                    -5 | 7 => Note::E,
                    -4 | 8 => Note::F,
                    -3 | 9 => Note::FSharp,
                    -2 | 10 => Note::G,
                    -1 | 11 => Note::GSharp,
                    0 => Note::A,
                    1 => Note::ASharp,
                    2 => Note::B,
                    _ => Note::A,
                }
            })
            .collect()
    }
}

/// Musical chord with root note and interval structure
#[derive(Clone, Debug)]
pub struct Chord {
    pub root: Note,
    pub intervals: Vec<i32>,
}

impl Chord {
    /// Create a major chord
    pub fn major(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 4, 7],
        }
    }
    
    /// Create a minor chord
    pub fn minor(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 3, 7],
        }
    }
    
    /// Create a dominant 7th chord
    pub fn dominant7(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 4, 7, 10],
        }
    }
    
    /// Create a minor 7th chord
    pub fn minor7(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 3, 7, 10],
        }
    }
    
    /// Get the notes that make up this chord
    pub fn notes(&self) -> Vec<Note> {
        let root_semitone = self.root.semitone_offset();
        self.intervals
            .iter()
            .map(|&interval| {
                let semitone = (root_semitone + interval + 120) % 12;
                match semitone {
                    -9 | 3 => Note::C,
                    -8 | 4 => Note::CSharp,
                    -7 | 5 => Note::D,
                    -6 | 6 => Note::DSharp,
                    -5 | 7 => Note::E,
                    -4 | 8 => Note::F,
                    -3 | 9 => Note::FSharp,
                    -2 | 10 => Note::G,
                    -1 | 11 => Note::GSharp,
                    0 => Note::A,
                    1 => Note::ASharp,
                    2 => Note::B,
                    _ => Note::A,
                }
            })
            .collect()
    }
}

/// Common chord progressions
#[derive(Clone, Debug, PartialEq)]
pub enum ChordProgression {
    Pop,         // I-V-vi-IV
    Jazz,        // ii-V-I
    Blues,       // I-IV-V
    Classical,   // I-vi-IV-V
}

impl ChordProgression {
    /// Get the chords for this progression in a given key
    pub fn get_chords(&self, key: Note) -> Vec<Chord> {
        let scale = Scale::Major.notes(key);
        
        match self {
            ChordProgression::Pop => vec![
                Chord::major(scale[0]),  // I
                Chord::major(scale[4]),  // V
                Chord::minor(scale[5]),  // vi
                Chord::major(scale[3]),  // IV
            ],
            ChordProgression::Jazz => vec![
                Chord::minor7(scale[1]), // ii7
                Chord::dominant7(scale[4]), // V7
                Chord::major(scale[0]),  // I
            ],
            ChordProgression::Blues => vec![
                Chord::major(scale[0]),  // I
                Chord::major(scale[3]),  // IV
                Chord::major(scale[4]),  // V
            ],
            ChordProgression::Classical => vec![
                Chord::major(scale[0]),  // I
                Chord::minor(scale[5]),  // vi
                Chord::major(scale[3]),  // IV
                Chord::major(scale[4]),  // V
            ],
        }
    }
}

/// Melody generation patterns
#[derive(Clone, Debug, PartialEq)]
pub enum MelodyPattern {
    Ascending,
    Descending,
    Arpeggio,
    ScaleRun,
    Random,
}

/// A melody composed of notes with durations
#[derive(Clone, Debug)]
pub struct Melody {
    pub notes: Vec<(Note, f32)>, // (note, duration in seconds)
}

impl Melody {
    /// Create a new empty melody
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }
    
    /// Create a melody from a scale with a given pattern
    pub fn from_scale(scale: Scale, root: Note, octave: i32, pattern: MelodyPattern, note_duration: f32) -> Self {
        let scale_notes = scale.notes(root);
        let mut notes = Vec::new();
        
        match pattern {
            MelodyPattern::Ascending => {
                for note in scale_notes {
                    notes.push((note, note_duration));
                }
            },
            MelodyPattern::Descending => {
                for note in scale_notes.iter().rev() {
                    notes.push((*note, note_duration));
                }
            },
            MelodyPattern::Arpeggio => {
                // Use first, third, fifth notes of scale (basic triad)
                if scale_notes.len() >= 5 {
                    notes.push((scale_notes[0], note_duration)); // Root
                    notes.push((scale_notes[2], note_duration)); // Third
                    notes.push((scale_notes[4], note_duration)); // Fifth
                    notes.push((scale_notes[2], note_duration)); // Third
                    notes.push((scale_notes[0], note_duration)); // Root
                }
            },
            MelodyPattern::ScaleRun => {
                // Quick ascending then descending
                for note in &scale_notes {
                    notes.push((*note, note_duration * 0.5));
                }
                for note in scale_notes.iter().rev().skip(1) {
                    notes.push((*note, note_duration * 0.5));
                }
            },
            MelodyPattern::Random => {
                // Simple pattern for now - just first few notes
                if scale_notes.len() >= 3 {
                    notes.push((scale_notes[0], note_duration));
                    notes.push((scale_notes[2], note_duration));
                    notes.push((scale_notes[1], note_duration));
                    notes.push((scale_notes[0], note_duration));
                }
            },
        }
        
        Self { notes }
    }
    
    /// Create a melody from a chord with a given pattern
    pub fn from_chord(chord: Chord, octave: i32, pattern: MelodyPattern, note_duration: f32) -> Self {
        let chord_notes = chord.notes();
        let mut notes = Vec::new();
        
        match pattern {
            MelodyPattern::Arpeggio => {
                // Standard arpeggio pattern
                for note in &chord_notes {
                    notes.push((*note, note_duration));
                }
                for note in chord_notes.iter().rev().skip(1) {
                    notes.push((*note, note_duration));
                }
            },
            MelodyPattern::Ascending => {
                for note in chord_notes {
                    notes.push((note, note_duration));
                }
            },
            MelodyPattern::Descending => {
                for note in chord_notes.iter().rev() {
                    notes.push((*note, note_duration));
                }
            },
            _ => {
                // Default to simple arpeggio
                for note in chord_notes {
                    notes.push((note, note_duration));
                }
            }
        }
        
        Self { notes }
    }
}

impl Default for Melody {
    fn default() -> Self {
        Self::new()
    }
}

/// Rhythm patterns for timing control
#[derive(Clone, Debug, PartialEq)]
pub enum RhythmPattern {
    Steady,      // Equal note durations
    Quick,       // Short, punchy notes
    Long,        // Sustained notes
    Notification, // Quick start, longer end
}

impl RhythmPattern {
    /// Get the duration values for this rhythm pattern
    pub fn get_durations(&self, base_duration: f32, note_count: usize) -> Vec<f32> {
        match self {
            RhythmPattern::Steady => vec![base_duration; note_count],
            RhythmPattern::Quick => vec![base_duration * 0.3; note_count],
            RhythmPattern::Long => vec![base_duration * 1.5; note_count],
            RhythmPattern::Notification => {
                let mut durations = vec![base_duration * 0.2; note_count.saturating_sub(1)];
                if note_count > 0 {
                    durations.push(base_duration * 1.0);
                }
                durations
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_frequency() {
        assert_eq!(Note::A.frequency(4), 440.0);
        assert!((Note::C.frequency(4) - 261.63).abs() < 0.01);
    }

    #[test]
    fn test_scale_notes() {
        let c_major = Scale::Major.notes(Note::C);
        assert_eq!(c_major[0], Note::C);
        assert_eq!(c_major[1], Note::D);
        assert_eq!(c_major[2], Note::E);
    }

    #[test]
    fn test_chord_notes() {
        let c_major_chord = Chord::major(Note::C);
        let notes = c_major_chord.notes();
        assert_eq!(notes[0], Note::C);
        assert_eq!(notes[1], Note::E);
        assert_eq!(notes[2], Note::G);
    }
}