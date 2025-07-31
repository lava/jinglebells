use jinglemaker::{
    JingleGenerator, 
    WaveForm, 
    Note, 
    Scale, 
    Melody, 
    MelodyPattern, 
    ChordProgression
};

fn main() -> Result<(), jinglemaker::JingleError> {
    println!("Jingle Maker - Rust Audio Generator with WAV Export");
    println!("==============================================");
    
    let generator = JingleGenerator::new();
    
    // Generate and export different types of jingles
    println!("Generating notification jingle...");
    let notification_samples = generator.create_notification_jingle(WaveForm::Sine);
    generator.export_to_wav(&notification_samples, "notification.wav")?;
    println!("✓ Exported notification.wav ({} samples)", notification_samples.len());
    
    println!("Generating success jingle...");
    let success_samples = generator.create_success_jingle(WaveForm::Triangle);
    generator.export_to_wav(&success_samples, "success.wav")?;
    println!("✓ Exported success.wav ({} samples)", success_samples.len());
    
    println!("Generating alert jingle...");
    let alert_samples = generator.create_alert_jingle(WaveForm::Square);
    generator.export_to_wav(&alert_samples, "alert.wav")?;
    println!("✓ Exported alert.wav ({} samples)", alert_samples.len());
    
    // Test different waveforms with the same melody
    println!("\nGenerating waveform variations...");
    let test_melody = Melody::from_scale(Scale::Major, Note::C, 4, MelodyPattern::Arpeggio, 0.3);
    
    let waveforms = [
        (WaveForm::Sine, "sine_arpeggio.wav"),
        (WaveForm::Triangle, "triangle_arpeggio.wav"),
        (WaveForm::Sawtooth, "sawtooth_arpeggio.wav"),
        (WaveForm::Square, "square_arpeggio.wav"),
    ];
    
    for (waveform, filename) in waveforms {
        let samples = generator.generate_melody_samples(&test_melody, 4, waveform);
        generator.export_to_wav(&samples, filename)?;
        println!("✓ Exported {} ({} samples)", filename, samples.len());
    }
    
    // Test chord progression export
    println!("\nGenerating chord progression jingle...");
    let chord_progression = ChordProgression::Pop.get_chords(Note::C);
    let mut chord_samples = Vec::new();
    
    for chord in chord_progression {
        let chord_melody = Melody::from_chord(chord, 4, MelodyPattern::Arpeggio, 0.2);
        let samples = generator.generate_melody_samples(&chord_melody, 4, WaveForm::Sine);
        chord_samples.extend(samples);
    }
    
    generator.export_to_wav(&chord_samples, "chord_progression.wav")?;
    println!("✓ Exported chord_progression.wav ({} samples)", chord_samples.len());
    
    println!("\n🎵 WAV export functionality is working!");
    println!("Generated files:");
    println!("  - notification.wav (C pentatonic arpeggio, sine wave)");
    println!("  - success.wav (C major ascending, triangle wave)");
    println!("  - alert.wav (double beep, square wave)");
    println!("  - sine_arpeggio.wav (C major arpeggio, sine wave)");
    println!("  - triangle_arpeggio.wav (C major arpeggio, triangle wave)");
    println!("  - sawtooth_arpeggio.wav (C major arpeggio, sawtooth wave)");
    println!("  - square_arpeggio.wav (C major arpeggio, square wave)");
    println!("  - chord_progression.wav (Pop progression I-V-vi-IV)");
    
    Ok(())
}