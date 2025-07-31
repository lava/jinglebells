use rodio::source::Source;
use std::time::Duration;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44100;
const A4_FREQUENCY: f32 = 440.0;

#[derive(Clone, Copy, Debug)]
pub enum Note {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}

impl Note {
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
    
    pub fn frequency(&self, octave: i32) -> f32 {
        let semitone_offset = self.semitone_offset() + (octave - 4) * 12;
        A4_FREQUENCY * 2.0_f32.powf(semitone_offset as f32 / 12.0)
    }
}

#[derive(Clone, Debug)]
pub enum Scale {
    Major,
    Minor,
    Pentatonic,
    Chromatic,
}

impl Scale {
    pub fn intervals(&self) -> Vec<i32> {
        match self {
            Scale::Major => vec![0, 2, 4, 5, 7, 9, 11],
            Scale::Minor => vec![0, 2, 3, 5, 7, 8, 10],
            Scale::Pentatonic => vec![0, 2, 4, 7, 9],
            Scale::Chromatic => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        }
    }
    
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

#[derive(Clone, Debug)]
pub struct Chord {
    pub root: Note,
    pub intervals: Vec<i32>,
}

impl Chord {
    pub fn major(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 4, 7],
        }
    }
    
    pub fn minor(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 3, 7],
        }
    }
    
    pub fn dominant7(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 4, 7, 10],
        }
    }
    
    pub fn minor7(root: Note) -> Self {
        Self {
            root,
            intervals: vec![0, 3, 7, 10],
        }
    }
    
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

#[derive(Clone, Debug)]
pub enum ChordProgression {
    Pop,         // I-V-vi-IV
    Jazz,        // ii-V-I
    Blues,       // I-IV-V
    Classical,   // I-vi-IV-V
}

impl ChordProgression {
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

#[derive(Clone, Debug)]
pub enum MelodyPattern {
    Ascending,
    Descending,
    Arpeggio,
    ScaleRun,
    Random,
}

#[derive(Clone, Debug)]
pub struct Melody {
    pub notes: Vec<(Note, f32)>, // (note, duration in seconds)
}

impl Melody {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }
    
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

#[derive(Clone, Debug)]
pub enum RhythmPattern {
    Steady,      // Equal note durations
    Quick,       // Short, punchy notes
    Long,        // Sustained notes
    Notification, // Quick start, longer end
}

impl RhythmPattern {
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

#[derive(Clone, Copy)]
pub enum WaveForm {
    Sine,
    Triangle,
    Sawtooth,
    Square,
}

#[derive(Clone, Copy)]
pub struct ADSR {
    pub attack: f32,  // seconds
    pub decay: f32,   // seconds
    pub sustain: f32, // amplitude level (0.0 - 1.0)
    pub release: f32, // seconds
}

impl Default for ADSR {
    fn default() -> Self {
        Self {
            attack: 0.1,
            decay: 0.1,
            sustain: 0.7,
            release: 0.2,
        }
    }
}

pub struct Oscillator {
    frequency: f32,
    waveform: WaveForm,
    adsr: ADSR,
    sample_rate: u32,
    current_sample: usize,
    total_duration: f32,
}

impl Oscillator {
    pub fn new(frequency: f32, waveform: WaveForm, duration: f32) -> Self {
        Self {
            frequency,
            waveform,
            adsr: ADSR::default(),
            sample_rate: SAMPLE_RATE,
            current_sample: 0,
            total_duration: duration,
        }
    }

    pub fn with_adsr(mut self, adsr: ADSR) -> Self {
        self.adsr = adsr;
        self
    }

    fn get_amplitude_envelope(&self, time: f32) -> f32 {
        let attack_time = self.adsr.attack;
        let decay_time = self.adsr.decay;
        let release_start = self.total_duration - self.adsr.release;

        if time < attack_time {
            // Attack phase
            time / attack_time
        } else if time < attack_time + decay_time {
            // Decay phase
            let decay_progress = (time - attack_time) / decay_time;
            1.0 - decay_progress * (1.0 - self.adsr.sustain)
        } else if time < release_start {
            // Sustain phase
            self.adsr.sustain
        } else {
            // Release phase
            let release_progress = (time - release_start) / self.adsr.release;
            self.adsr.sustain * (1.0 - release_progress)
        }
    }

    fn generate_wave(&self, time: f32) -> f32 {
        let phase = time * self.frequency * 2.0 * PI;
        
        match self.waveform {
            WaveForm::Sine => phase.sin(),
            WaveForm::Triangle => {
                let normalized_phase = (phase / (2.0 * PI)) % 1.0;
                if normalized_phase < 0.5 {
                    4.0 * normalized_phase - 1.0
                } else {
                    3.0 - 4.0 * normalized_phase
                }
            },
            WaveForm::Sawtooth => {
                let normalized_phase = (phase / (2.0 * PI)) % 1.0;
                2.0 * normalized_phase - 1.0
            },
            WaveForm::Square => {
                if phase.sin() >= 0.0 { 1.0 } else { -1.0 }
            },
        }
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let time = self.current_sample as f32 / self.sample_rate as f32;
        
        if time >= self.total_duration {
            return None;
        }

        let wave_value = self.generate_wave(time);
        let envelope = self.get_amplitude_envelope(time);
        let sample = wave_value * envelope * 0.3; // Reduce volume to prevent clipping

        self.current_sample += 1;
        Some(sample)
    }
}

impl Source for Oscillator {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(self.total_duration))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Jingle Maker - Rust Audio Generator");
    println!("Musical Pattern Generators Implemented!");
    
    // Test scale generation
    let c_major_scale = Scale::Major.notes(Note::C);
    println!("C Major scale: {:?}", c_major_scale);
    
    // Test chord generation
    let c_major_chord = Chord::major(Note::C);
    println!("C Major chord notes: {:?}", c_major_chord.notes());
    
    // Test melody generation
    let melody = Melody::from_scale(Scale::Pentatonic, Note::C, 4, MelodyPattern::Arpeggio, 0.25);
    println!("Generated melody with {} notes", melody.notes.len());
    
    // Test frequency calculation
    let c4_freq = Note::C.frequency(4);
    let a4_freq = Note::A.frequency(4);
    println!("C4 frequency: {:.2}Hz, A4 frequency: {:.2}Hz", c4_freq, a4_freq);
    
    // Generate a sample using the first note of the melody
    if let Some((note, duration)) = melody.notes.first() {
        let freq = note.frequency(4);
        println!("Testing oscillator with {}Hz for {:.2}s", freq, duration);
        
        let oscillator = Oscillator::new(freq, WaveForm::Sine, *duration);
        let samples: Vec<f32> = oscillator.take(1000).collect();
        println!("Generated {} samples", samples.len());
        println!("First few samples: {:?}", &samples[0..5]);
    }
    
    println!("Musical pattern generators are working!");
    Ok(())
}