//! Audio effects processing

use rodio::source::Source;
use std::collections::VecDeque;
use std::time::Duration;
use crate::SAMPLE_RATE;

/// Simple delay buffer for creating echo and reverb effects
#[derive(Clone)]
pub struct DelayBuffer {
    buffer: VecDeque<f32>,
    max_delay_samples: usize,
    delay_samples: usize,
    feedback: f32,
    mix: f32,
}

impl DelayBuffer {
    /// Create a new delay buffer
    /// - delay_ms: delay time in milliseconds
    /// - feedback: amount of delayed signal fed back (0.0 - 0.95)
    /// - mix: wet/dry mix (0.0 = dry only, 1.0 = wet only)
    pub fn new(delay_ms: f32, feedback: f32, mix: f32) -> Self {
        let delay_samples = ((delay_ms / 1000.0) * SAMPLE_RATE as f32) as usize;
        let max_delay_samples = delay_samples.max(1);
        
        Self {
            buffer: VecDeque::with_capacity(max_delay_samples),
            max_delay_samples,
            delay_samples,
            feedback: feedback.clamp(0.0, 0.95),
            mix: mix.clamp(0.0, 1.0),
        }
    }

    /// Process a single sample through the delay buffer
    pub fn process_sample(&mut self, input: f32) -> f32 {
        // Initialize buffer with zeros if needed
        while self.buffer.len() < self.max_delay_samples {
            self.buffer.push_back(0.0);
        }

        // Get delayed sample
        let delayed_sample = if self.delay_samples < self.buffer.len() {
            self.buffer[self.buffer.len() - self.delay_samples - 1]
        } else {
            0.0
        };

        // Calculate output with feedback
        let output_sample = input + delayed_sample * self.feedback;
        
        // Add to buffer
        self.buffer.push_back(output_sample);
        if self.buffer.len() > self.max_delay_samples {
            self.buffer.pop_front();
        }

        // Mix wet and dry signals
        input * (1.0 - self.mix) + delayed_sample * self.mix
    }
}

/// Echo effect wrapper for any Source
pub struct Echo<S>
where
    S: Source<Item = f32>,
{
    source: S,
    delay_buffer: DelayBuffer,
}

impl<S> Echo<S>
where
    S: Source<Item = f32>,
{
    /// Create a new echo effect
    /// - source: input audio source
    /// - delay_ms: echo delay in milliseconds
    /// - feedback: echo feedback amount (0.0 - 0.95)
    /// - mix: wet/dry mix (0.0 - 1.0)
    pub fn new(source: S, delay_ms: f32, feedback: f32, mix: f32) -> Self {
        Self {
            source,
            delay_buffer: DelayBuffer::new(delay_ms, feedback, mix),
        }
    }
}

impl<S> Iterator for Echo<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|sample| {
            self.delay_buffer.process_sample(sample)
        })
    }
}

impl<S> Source for Echo<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

/// Simple reverb effect using multiple delay lines
pub struct Reverb<S>
where
    S: Source<Item = f32>,
{
    source: S,
    delay_buffers: Vec<DelayBuffer>,
}

impl<S> Reverb<S>
where
    S: Source<Item = f32>,
{
    /// Create a new reverb effect with multiple delay lines
    pub fn new(source: S, room_size: f32, damping: f32, mix: f32) -> Self {
        // Create multiple delay lines with different delays for natural reverb
        let delays = [
            29.0, 37.0, 41.0, 43.0, 47.0, 53.0, 59.0, 61.0
        ];
        
        let delay_buffers = delays.iter().map(|&delay_ms| {
            let actual_delay = delay_ms * room_size;
            let feedback = damping * 0.6; // Limit feedback to prevent runaway
            DelayBuffer::new(actual_delay, feedback, mix * 0.125) // Divide mix by number of delays
        }).collect();

        Self {
            source,
            delay_buffers,
        }
    }

    /// Create a small room reverb (quick, subtle)
    pub fn small_room(source: S) -> Self {
        Self::new(source, 0.5, 0.3, 0.2)
    }

    /// Create a large hall reverb (long, lush)
    pub fn large_hall(source: S) -> Self {
        Self::new(source, 1.5, 0.5, 0.4)
    }
}

impl<S> Iterator for Reverb<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|sample| {
            let mut output = sample;
            
            // Process through all delay buffers
            for delay_buffer in &mut self.delay_buffers {
                output += delay_buffer.process_sample(sample);
            }
            
            // Normalize to prevent clipping
            output * 0.7
        })
    }
}

impl<S> Source for Reverb<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

/// Simple low-pass filter for smoothing audio signals
#[derive(Clone)]
pub struct LowPassFilter {
    #[allow(dead_code)]
    cutoff_frequency: f32,
    #[allow(dead_code)]
    sample_rate: f32,
    alpha: f32,
    previous_output: f32,
}

impl LowPassFilter {
    /// Create a new low-pass filter
    /// - cutoff_frequency: frequency above which signals are attenuated (Hz)
    /// - sample_rate: audio sample rate (Hz)
    pub fn new(cutoff_frequency: f32, sample_rate: f32) -> Self {
        // Calculate filter coefficient
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_frequency);
        let dt = 1.0 / sample_rate;
        let alpha = dt / (rc + dt);

        Self {
            cutoff_frequency,
            sample_rate,
            alpha,
            previous_output: 0.0,
        }
    }

    /// Process a single sample through the filter
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.alpha * input + (1.0 - self.alpha) * self.previous_output;
        self.previous_output = output;
        output
    }

    /// Reset the filter state
    pub fn reset(&mut self) {
        self.previous_output = 0.0;
    }
}

/// Low-pass filter wrapper for any Source
pub struct LowPass<S>
where
    S: Source<Item = f32>,
{
    source: S,
    filter: LowPassFilter,
}

impl<S> LowPass<S>
where
    S: Source<Item = f32>,
{
    /// Create a new low-pass filter effect
    pub fn new(source: S, cutoff_frequency: f32) -> Self {
        Self {
            filter: LowPassFilter::new(cutoff_frequency, SAMPLE_RATE as f32),
            source,
        }
    }

    /// Create a smooth/warm low-pass filter (removes harsh frequencies)
    pub fn smooth(source: S) -> Self {
        Self::new(source, 4000.0) // 4kHz cutoff
    }

    /// Create a muffled low-pass filter (simulates distance/walls)
    pub fn muffled(source: S) -> Self {
        Self::new(source, 1000.0) // 1kHz cutoff
    }
}

impl<S> Iterator for LowPass<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|sample| {
            self.filter.process_sample(sample)
        })
    }
}

impl<S> Source for LowPass<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

/// Automatic Gain Control (AGC) for dynamic range compression
#[derive(Clone)]
pub struct AutomaticGainControl {
    target_level: f32,
    attack_time: f32,
    release_time: f32,
    current_gain: f32,
    envelope_follower: f32,
    sample_rate: f32,
}

impl AutomaticGainControl {
    /// Create a new AGC
    /// - target_level: desired output level (0.0 - 1.0)
    /// - attack_time: time to reduce gain when signal is loud (seconds)
    /// - release_time: time to increase gain when signal is quiet (seconds)
    pub fn new(target_level: f32, attack_time: f32, release_time: f32, sample_rate: f32) -> Self {
        Self {
            target_level: target_level.clamp(0.0, 1.0),
            attack_time,
            release_time,
            current_gain: 1.0,
            envelope_follower: 0.0,
            sample_rate,
        }
    }

    /// Process a single sample through the AGC
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let input_level = input.abs();
        
        // Envelope follower (peak detector with decay)
        let attack_coeff = (-1.0 / (self.attack_time * self.sample_rate)).exp();
        let release_coeff = (-1.0 / (self.release_time * self.sample_rate)).exp();
        
        if input_level > self.envelope_follower {
            // Attack: fast response to loud signals
            self.envelope_follower = input_level + (self.envelope_follower - input_level) * attack_coeff;
        } else {
            // Release: slow response to quiet signals
            self.envelope_follower = input_level + (self.envelope_follower - input_level) * release_coeff;
        }

        // Calculate required gain
        let required_gain = if self.envelope_follower > 0.0001 {
            self.target_level / self.envelope_follower
        } else {
            1.0
        };

        // Smooth gain changes
        let gain_diff = required_gain - self.current_gain;
        let gain_coeff = if gain_diff > 0.0 { release_coeff } else { attack_coeff };
        self.current_gain += gain_diff * (1.0 - gain_coeff);

        // Limit gain to prevent excessive amplification
        self.current_gain = self.current_gain.clamp(0.1, 10.0);

        input * self.current_gain
    }
}

/// AGC wrapper for any Source
pub struct AGC<S>
where
    S: Source<Item = f32>,
{
    source: S,
    agc: AutomaticGainControl,
}

impl<S> AGC<S>
where
    S: Source<Item = f32>,
{
    /// Create a new AGC effect
    pub fn new(source: S, target_level: f32, attack_time: f32, release_time: f32) -> Self {
        Self {
            agc: AutomaticGainControl::new(target_level, attack_time, release_time, SAMPLE_RATE as f32),
            source,
        }
    }

    /// Create a gentle AGC for musical content
    pub fn gentle(source: S) -> Self {
        Self::new(source, 0.7, 0.003, 0.1) // Fast attack, slow release
    }

    /// Create a limiting AGC for preventing clipping
    pub fn limiter(source: S) -> Self {
        Self::new(source, 0.95, 0.001, 0.05) // Very fast attack, quick release
    }
}

impl<S> Iterator for AGC<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|sample| {
            self.agc.process_sample(sample)
        })
    }
}

impl<S> Source for AGC<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> {
        self.source.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.source.total_duration()
    }
}

/// Normalize audio samples to a target peak level
pub fn normalize_samples(samples: &mut [f32], target_peak: f32) {
    if samples.is_empty() {
        return;
    }

    // Find peak level
    let peak = samples.iter()
        .map(|&x| x.abs())
        .fold(0.0f32, |acc, x| acc.max(x));

    if peak > 0.0001 {
        // Calculate normalization factor
        let gain = target_peak / peak;
        
        // Apply gain to all samples
        for sample in samples.iter_mut() {
            *sample *= gain;
        }
    }
}

/// Peak normalize a vector of samples
pub fn peak_normalize(samples: Vec<f32>, target_peak: f32) -> Vec<f32> {
    let mut normalized = samples;
    normalize_samples(&mut normalized, target_peak);
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::{Oscillator, WaveForm};

    #[test]
    fn test_delay_buffer() {
        let mut delay = DelayBuffer::new(10.0, 0.5, 0.3); // 10ms delay
        let output1 = delay.process_sample(1.0);
        let output2 = delay.process_sample(0.0);
        
        // First sample should be mostly dry
        assert!(output1 > 0.5);
        // Later samples should have some delay
        assert!(output2.abs() < 1.0);
    }

    #[test]
    fn test_echo_effect() {
        let osc = Oscillator::new(440.0, WaveForm::Sine, 0.1);
        let mut echo = Echo::new(osc, 50.0, 0.3, 0.2);
        
        let first_sample = echo.next();
        assert!(first_sample.is_some());
        assert!(first_sample.unwrap().abs() <= 1.0);
    }

    #[test]
    fn test_reverb_effect() {
        let osc = Oscillator::new(440.0, WaveForm::Sine, 0.1);
        let mut reverb = Reverb::small_room(osc);
        
        let first_sample = reverb.next();
        assert!(first_sample.is_some());
        assert!(first_sample.unwrap().abs() <= 1.0);
    }

    #[test]
    fn test_lowpass_filter() {
        let mut filter = LowPassFilter::new(1000.0, SAMPLE_RATE as f32);
        let output1 = filter.process_sample(1.0);
        let output2 = filter.process_sample(0.0);
        
        // Filter should smooth the signal
        assert!(output1 > 0.0 && output1 < 1.0);
        assert!(output2 > 0.0); // Should retain some previous value
    }

    #[test]
    fn test_lowpass_effect() {
        let osc = Oscillator::new(440.0, WaveForm::Square, 0.1); // Square wave has harmonics
        let mut lowpass = LowPass::smooth(osc);
        
        let first_sample = lowpass.next();
        assert!(first_sample.is_some());
        assert!(first_sample.unwrap().abs() <= 1.0);
    }

    #[test]
    fn test_agc() {
        let mut agc = AutomaticGainControl::new(0.5, 0.01, 0.1, SAMPLE_RATE as f32);
        
        // Test that AGC produces reasonable output
        let output1 = agc.process_sample(1.0);
        let output2 = agc.process_sample(0.1);
        
        // AGC should produce finite outputs
        assert!(output1.is_finite());
        assert!(output2.is_finite());
        assert!(output1.abs() <= 10.0); // Within reasonable bounds
        assert!(output2.abs() <= 10.0);
    }

    #[test]
    fn test_agc_effect() {
        let osc = Oscillator::new(440.0, WaveForm::Sine, 0.1);
        let mut agc = AGC::gentle(osc);
        
        let first_sample = agc.next();
        assert!(first_sample.is_some());
        assert!(first_sample.unwrap().abs() <= 1.0);
    }

    #[test]
    fn test_normalize_samples() {
        let mut samples = vec![0.1, -0.5, 0.3, -0.8];
        normalize_samples(&mut samples, 0.9);
        
        // Peak should now be 0.9
        let peak = samples.iter().map(|&x| x.abs()).fold(0.0f32, |acc, x| acc.max(x));
        assert!((peak - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_peak_normalize() {
        let samples = vec![0.2, -1.0, 0.5, -0.3];
        let normalized = peak_normalize(samples, 0.8);
        
        // Peak should now be 0.8
        let peak = normalized.iter().map(|&x| x.abs()).fold(0.0f32, |acc, x| acc.max(x));
        assert!((peak - 0.8).abs() < 0.001);
    }
}