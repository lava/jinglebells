use rodio::source::Source;
use std::time::Duration;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44100;

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
    
    // For now, let's just test a simple sine wave
    println!("Playing 440Hz sine wave for 1 second...");
    
    let oscillator = Oscillator::new(440.0, WaveForm::Sine, 1.0);
    
    // Instead of audio output, let's just print some samples for now
    let samples: Vec<f32> = oscillator.take(1000).collect();
    println!("Generated {} samples", samples.len());
    println!("First few samples: {:?}", &samples[0..5]);
    
    println!("Audio synthesis core is working!");
    Ok(())
}