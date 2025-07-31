use clap::{Parser, ValueEnum};
use jinglemaker::{JingleGenerator, WaveForm};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jinglemaker")]
#[command(about = "A CLI jingle generator using Rust and Rodio")]
#[command(version = "0.1.0")]
struct Cli {
    /// The type of jingle to generate
    #[arg(value_enum)]
    preset: Preset,
    
    /// Duration in seconds (0.5-3.0) - NOTE: Currently not implemented, presets use fixed durations
    #[arg(short, long, default_value = "1.0")]
    duration: f32,
    
    /// Base frequency in Hz (200-800) - NOTE: Currently not implemented, presets use fixed frequencies  
    #[arg(short, long, default_value = "440.0")]
    frequency: f32,
    
    /// Waveform type
    #[arg(short, long, value_enum, default_value = "sine")]
    waveform: WaveFormArg,
    
    /// Output file path
    #[arg(short, long, default_value = "jingle.wav")]
    output: PathBuf,
    
    /// Number of variations to generate
    #[arg(short, long, default_value = "1")]
    count: u32,
    
    /// Seed for reproducible generation
    #[arg(short, long)]
    seed: Option<u64>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Preset {
    /// Gentle notification sound
    Notification,
    /// Attention-grabbing alert
    Alert,
    /// Pleasant success chime
    Success,
    /// Warning error sound
    Error,
    /// System startup jingle
    Startup,
    /// System shutdown sound
    Shutdown,
    /// Message received notification
    Message,
    /// Task completion sound
    Completion,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum WaveFormArg {
    Sine,
    Triangle,
    Sawtooth,
    Square,
}

impl From<WaveFormArg> for WaveForm {
    fn from(arg: WaveFormArg) -> Self {
        match arg {
            WaveFormArg::Sine => WaveForm::Sine,
            WaveFormArg::Triangle => WaveForm::Triangle,
            WaveFormArg::Sawtooth => WaveForm::Sawtooth,
            WaveFormArg::Square => WaveForm::Square,
        }
    }
}

impl Preset {
    fn generate_samples(&self, generator: &JingleGenerator, waveform: WaveForm) -> Vec<f32> {
        match self {
            Preset::Notification => generator.create_notification_jingle(waveform),
            Preset::Alert => generator.create_alert_jingle(waveform),
            Preset::Success => generator.create_success_jingle(waveform),
            Preset::Error => generator.create_error_jingle(waveform),
            Preset::Startup => generator.create_startup_jingle(waveform),
            Preset::Shutdown => generator.create_shutdown_jingle(waveform),
            Preset::Message => generator.create_message_jingle(waveform),
            Preset::Completion => generator.create_completion_jingle(waveform),
        }
    }
    
    fn description(&self) -> &'static str {
        match self {
            Preset::Notification => "Gentle notification sound",
            Preset::Alert => "Attention-grabbing alert",
            Preset::Success => "Pleasant success chime",
            Preset::Error => "Warning error sound",
            Preset::Startup => "System startup jingle",
            Preset::Shutdown => "System shutdown sound",
            Preset::Message => "Message received notification",
            Preset::Completion => "Task completion sound",
        }
    }
}

fn main() -> Result<(), jinglemaker::JingleError> {
    let cli = Cli::parse();
    
    // Validate parameters
    if cli.count == 0 || cli.count > 100 {
        eprintln!("Error: Count must be between 1 and 100");
        std::process::exit(1);
    }
    
    // Note: duration and frequency parameters are currently not implemented
    if cli.duration != 1.0 || cli.frequency != 440.0 {
        println!("Note: Duration and frequency parameters are not currently implemented (presets use fixed values)");
    }
    
    let generator = JingleGenerator::new();
    let waveform = WaveForm::from(cli.waveform);
    
    // Note: seed parameter is accepted but not currently used since presets are deterministic
    if cli.seed.is_some() {
        println!("Note: Seed parameter is not currently implemented (presets are deterministic)");
    }
    
    println!("Generating {} jingle(s)...", cli.count);
    println!("Preset: {} ({})", format!("{:?}", cli.preset).to_lowercase(), cli.preset.description());
    println!("Waveform: {:?}", cli.waveform);
    
    for i in 0..cli.count {
        let samples = cli.preset.generate_samples(&generator, waveform);
        
        let output_path = if cli.count == 1 {
            cli.output.clone()
        } else {
            let stem = cli.output.file_stem().unwrap_or_default().to_string_lossy();
            let extension = cli.output.extension().unwrap_or_default().to_string_lossy();
            let parent = cli.output.parent().unwrap_or_else(|| std::path::Path::new("."));
            parent.join(format!("{}_{}.{}", stem, i + 1, extension))
        };
        
        generator.export_to_wav(&samples, &output_path)?;
        println!("✓ Generated {} ({} samples)", output_path.display(), samples.len());
    }
    
    println!("\n🎵 Jingle generation complete!");
    
    Ok(())
}