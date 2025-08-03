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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
        #[arg(short, long, default_value = "output.wav")]
        output: PathBuf,
        
        /// Number of variations to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        
        /// Seed for reproducible generation
        #[arg(short, long)]
        seed: Option<u64>,
        
        /// Generate file only without playing
        #[arg(short, long)]
        generate_only: bool,
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
    fn generate_samples(&self, generator: &mut JingleGenerator) -> Vec<f32> {
        let (_, _, _, duration, frequency, _) = self.get_params();
        
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
    
    fn get_params(&self) -> (PathBuf, u32, Option<u64>, f32, f32, bool) {
        match self {
            Preset::Notification { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Alert { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Success { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Error { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Startup { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Shutdown { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Message { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
            Preset::Completion { output, count, seed, duration, frequency, generate_only, .. } => (output.clone(), *count, *seed, *duration, *frequency, *generate_only),
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

fn play_samples(samples: &[f32]) -> Result<(), jinglemaker::JingleError> {
    // Get output stream handle
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|e| jinglemaker::JingleError::PlaybackError(e.to_string()))?;
    
    // Create sink connected to the stream
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    
    // Convert samples to the format rodio expects
    let source = rodio::buffer::SamplesBuffer::new(1, jinglemaker::SAMPLE_RATE, samples.to_vec());
    
    // Add the source to the sink
    sink.append(source);
    
    // Block until playback is complete
    sink.sleep_until_end();
    
    Ok(())
}

fn main() -> Result<(), jinglemaker::JingleError> {
    let cli = Cli::parse();
    
    let (output, count, seed, duration, frequency, generate_only) = cli.preset.get_params();
    let waveform = cli.preset.get_waveform();
    
    // Validate parameters
    if count == 0 || count > 100 {
        eprintln!("Error: Count must be between 1 and 100");
        std::process::exit(1);
    }
    
    let _generator = if let Some(seed_value) = seed {
        JingleGenerator::with_seed(seed_value)
    } else {
        JingleGenerator::new()
    };
    
    // Build the CLI command that would generate the same audio
    let mut cmd = format!("jinglemaker {}", cli.preset.name());
    
    // Add non-default parameters to the command
    if duration != 1.0 {
        cmd.push_str(&format!(" --duration {}", duration));
    }
    if frequency != 440.0 {
        cmd.push_str(&format!(" --frequency {}", frequency));
    }
    if waveform != WaveFormArg::Sine && !is_default_waveform_for_preset(&cli.preset, waveform) {
        cmd.push_str(&format!(" --waveform {:?}", waveform).to_lowercase());
    }
    if output.to_string_lossy() != "output.wav" {
        cmd.push_str(&format!(" --output {}", output.display()));
    }
    if count != 1 {
        cmd.push_str(&format!(" --count {}", count));
    }
    if let Some(seed_value) = seed {
        cmd.push_str(&format!(" --seed {}", seed_value));
    }
    if generate_only {
        cmd.push_str(" --generate-only");
    }
    
    // Print the command
    println!("{}", cmd);
    
    Ok(())
}

fn is_default_waveform_for_preset(preset: &Preset, waveform: WaveFormArg) -> bool {
    match preset {
        Preset::Notification { .. } => waveform == WaveFormArg::Sine,
        Preset::Alert { .. } => waveform == WaveFormArg::Square,
        Preset::Success { .. } => waveform == WaveFormArg::Triangle,
        Preset::Error { .. } => waveform == WaveFormArg::Sawtooth,
        Preset::Startup { .. } => waveform == WaveFormArg::Sine,
        Preset::Shutdown { .. } => waveform == WaveFormArg::Sine,
        Preset::Message { .. } => waveform == WaveFormArg::Sine,
        Preset::Completion { .. } => waveform == WaveFormArg::Sine,
    }
}