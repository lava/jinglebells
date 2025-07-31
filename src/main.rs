use clap::{Parser, Subcommand, ValueEnum};
use jinglemaker::{JingleGenerator, WaveForm};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jinglemaker")]
#[command(about = "A CLI jingle generator using Rust and Rodio")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    preset: Preset,
}

#[derive(Subcommand, Debug)]
enum Preset {
    /// Generate a gentle notification sound
    Notification {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "sine")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "notification.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate an attention-grabbing alert
    Alert {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "square")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "alert.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate a pleasant success chime
    Success {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "triangle")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "success.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate a warning error sound
    Error {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "sawtooth")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "error.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate a system startup jingle
    Startup {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "sine")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "startup.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate a system shutdown sound
    Shutdown {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "sine")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "shutdown.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate a message received notification
    Message {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "sine")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "message.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
    /// Generate a task completion sound
    Completion {
        /// Duration in seconds (0.5-3.0)
        #[arg(short, long, default_value = "1.0")]
        duration: f32,
        
        /// Base frequency in Hz (200-800)
        #[arg(short, long, default_value = "440.0")]
        frequency: f32,
        
        /// Waveform type
        #[arg(short, long, value_enum, default_value = "sine")]
        waveform: WaveFormArg,
        
        /// Output file path
        #[arg(short, long, default_value = "completion.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
    },
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
    fn generate_samples(&self, generator: &JingleGenerator) -> Vec<f32> {
        let (_, _, _, duration, frequency) = self.get_params();
        
        // Convert values to None if they are the defaults (meaning user didn't specify them)
        let duration_opt = if duration != 1.0 { Some(duration) } else { None };
        let frequency_opt = if frequency != 440.0 { Some(frequency) } else { None };
        
        match self {
            Preset::Notification { waveform, .. } => generator.create_notification_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Alert { waveform, .. } => generator.create_alert_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Success { waveform, .. } => generator.create_success_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Error { waveform, .. } => generator.create_error_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Startup { waveform, .. } => generator.create_startup_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Shutdown { waveform, .. } => generator.create_shutdown_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Message { waveform, .. } => generator.create_message_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
            Preset::Completion { waveform, .. } => generator.create_completion_jingle(WaveForm::from(*waveform), duration_opt, frequency_opt),
        }
    }
    
    fn get_params(&self) -> (PathBuf, u32, Option<u64>, f32, f32) {
        match self {
            Preset::Notification { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Alert { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Success { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Error { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Startup { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Shutdown { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Message { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
            Preset::Completion { output, count, seed, duration, frequency, .. } => (output.clone(), *count, *seed, *duration, *frequency),
        }
    }
    
    fn get_waveform(&self) -> WaveFormArg {
        match self {
            Preset::Notification { waveform, .. } => *waveform,
            Preset::Alert { waveform, .. } => *waveform,
            Preset::Success { waveform, .. } => *waveform,
            Preset::Error { waveform, .. } => *waveform,
            Preset::Startup { waveform, .. } => *waveform,
            Preset::Shutdown { waveform, .. } => *waveform,
            Preset::Message { waveform, .. } => *waveform,
            Preset::Completion { waveform, .. } => *waveform,
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            Preset::Notification { .. } => "notification",
            Preset::Alert { .. } => "alert",
            Preset::Success { .. } => "success",
            Preset::Error { .. } => "error",
            Preset::Startup { .. } => "startup",
            Preset::Shutdown { .. } => "shutdown",
            Preset::Message { .. } => "message",
            Preset::Completion { .. } => "completion",
        }
    }
    
    fn description(&self) -> &'static str {
        match self {
            Preset::Notification { .. } => "Gentle notification sound",
            Preset::Alert { .. } => "Attention-grabbing alert",
            Preset::Success { .. } => "Pleasant success chime",
            Preset::Error { .. } => "Warning error sound",
            Preset::Startup { .. } => "System startup jingle",
            Preset::Shutdown { .. } => "System shutdown sound",
            Preset::Message { .. } => "Message received notification",
            Preset::Completion { .. } => "Task completion sound",
        }
    }
}

fn main() -> Result<(), jinglemaker::JingleError> {
    let cli = Cli::parse();
    
    let (output, count, seed, _duration, _frequency) = cli.preset.get_params();
    let waveform = cli.preset.get_waveform();
    
    // Validate parameters
    if count == 0 || count > 100 {
        eprintln!("Error: Count must be between 1 and 100");
        std::process::exit(1);
    }
    
    
    // Note: seed parameter is accepted but not currently used since presets are deterministic
    if seed.is_some() {
        println!("Note: Seed parameter is not currently implemented (presets are deterministic)");
    }
    
    let generator = JingleGenerator::new();
    
    println!("Generating {} jingle(s)...", count);
    println!("Preset: {} ({})", cli.preset.name(), cli.preset.description());
    println!("Waveform: {:?}", waveform);
    
    for i in 0..count {
        let samples = cli.preset.generate_samples(&generator);
        
        let output_path = if count == 1 {
            output.clone()
        } else {
            let stem = output.file_stem().unwrap_or_default().to_string_lossy();
            let extension = output.extension().unwrap_or_default().to_string_lossy();
            let parent = output.parent().unwrap_or_else(|| std::path::Path::new("."));
            parent.join(format!("{}_{}.{}", stem, i + 1, extension))
        };
        
        generator.export_to_wav(&samples, &output_path)?;
        println!("✓ Generated {} ({} samples)", output_path.display(), samples.len());
    }
    
    println!("\n🎵 Jingle generation complete!");
    
    Ok(())
}