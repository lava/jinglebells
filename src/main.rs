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
    
}

fn play_samples(samples: &[f32]) -> Result<(), jinglemaker::JingleError> {
    // Get output stream handle
    let mut stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|e| jinglemaker::JingleError::PlaybackError(e.to_string()))?;
    
    // Disable drop logging to avoid interfering with CLI output
    stream_handle.log_on_drop(false);
    
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

fn print_file_write_command(preset: &Preset, seed: u64) {
    let current_exe = std::env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from("jinglemaker"));
    let exe_name = current_exe.file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("jinglemaker"))
        .to_string_lossy();
    
    let mut cmd_args = Vec::new();
    
    // Add the preset name
    let preset_name = match preset {
        Preset::Notification { .. } => "notification",
        Preset::Alert { .. } => "alert", 
        Preset::Success { .. } => "success",
        Preset::Error { .. } => "error",
        Preset::Startup { .. } => "startup",
        Preset::Shutdown { .. } => "shutdown",
        Preset::Message { .. } => "message",
        Preset::Completion { .. } => "completion",
    };
    cmd_args.push(preset_name.to_string());
    
    // Extract parameters from the preset
    let (output, count, _, duration, frequency, _) = preset.get_params();
    
    // Add all the parameters that reproduce the exact same sound
    if duration != 1.0 {
        cmd_args.push("--duration".to_string());
        cmd_args.push(duration.to_string());
    }
    
    if frequency != 440.0 {
        cmd_args.push("--frequency".to_string());
        cmd_args.push(frequency.to_string());
    }
    
    // Add waveform if it's not the default for this preset
    let waveform_arg = match preset {
        Preset::Notification { waveform, .. } => if *waveform != WaveFormArg::Sine { Some(format!("{:?}", waveform).to_lowercase()) } else { None },
        Preset::Alert { waveform, .. } => if *waveform != WaveFormArg::Square { Some(format!("{:?}", waveform).to_lowercase()) } else { None },
        Preset::Success { waveform, .. } => if *waveform != WaveFormArg::Triangle { Some(format!("{:?}", waveform).to_lowercase()) } else { None },
        Preset::Error { waveform, .. } => if *waveform != WaveFormArg::Sawtooth { Some(format!("{:?}", waveform).to_lowercase()) } else { None },
        Preset::Startup { waveform, .. } | Preset::Shutdown { waveform, .. } | Preset::Message { waveform, .. } | Preset::Completion { waveform, .. } => {
            if *waveform != WaveFormArg::Sine { Some(format!("{:?}", waveform).to_lowercase()) } else { None }
        },
    };
    
    if let Some(wf) = waveform_arg {
        cmd_args.push("--waveform".to_string());
        cmd_args.push(wf);
    }
    
    // Always add the seed to ensure reproducibility
    cmd_args.push("--seed".to_string());
    cmd_args.push(seed.to_string());
    
    // Add count if not 1
    if count != 1 {
        cmd_args.push("--count".to_string());
        cmd_args.push(count.to_string());
    }
    
    // Add explicit output path
    cmd_args.push("--output".to_string());
    cmd_args.push(output.to_string_lossy().to_string());
    
    // Add --generate-only flag
    cmd_args.push("--generate-only".to_string());
    
    // Print the command
    println!("To write this sound to a file, run:");
    println!("{} {}", exe_name, cmd_args.join(" "));
}

fn main() -> Result<(), jinglemaker::JingleError> {
    let cli = Cli::parse();
    
    let (output, count, seed, _duration, _frequency, generate_only) = cli.preset.get_params();
    
    // Validate parameters
    if count == 0 || count > 100 {
        eprintln!("Error: Count must be between 1 and 100");
        std::process::exit(1);
    }
    
    // Always use a seed - generate one if not provided
    let actual_seed = seed.unwrap_or_else(|| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        hasher.finish()
    });
    
    let mut generator = JingleGenerator::with_seed(actual_seed);
    
    for i in 0..count {
        let samples = cli.preset.generate_samples(&mut generator);
        
        // Save to file if generate_only is specified
        if generate_only {
            if count > 1 {
                let file_stem = output.file_stem().unwrap_or_default().to_string_lossy();
                let file_ext = output.extension().unwrap_or_default().to_string_lossy();
                let numbered_output = if file_ext.is_empty() {
                    format!("{}_{}", file_stem, i)
                } else {
                    format!("{}_{}.{}", file_stem, i, file_ext)
                };
                let numbered_path = output.with_file_name(&numbered_output);
                generator.export_to_wav(&samples, &numbered_path)?;
            } else {
                generator.export_to_wav(&samples, &output)?;
            }
        } else {
            // Play audio by default
            play_samples(&samples)?;
            
            // Print the command to write this sound to a file
            print_file_write_command(&cli.preset, actual_seed);
        }
    }
    
    Ok(())
}

