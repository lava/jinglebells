//! Error types for the jingle maker library

use std::fmt;

/// Main error type for jingle generation operations
#[derive(Debug)]
pub enum JingleError {
    /// Audio generation error
    AudioError(String),
    /// File I/O error
    IoError(std::io::Error),
    /// WAV encoding error
    WavError(hound::Error),
    /// Invalid parameter error
    InvalidParameter(String),
    /// Audio playback error
    PlaybackError(String),
}

impl fmt::Display for JingleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JingleError::AudioError(msg) => write!(f, "Audio generation error: {}", msg),
            JingleError::IoError(err) => write!(f, "I/O error: {}", err),
            JingleError::WavError(err) => write!(f, "WAV encoding error: {}", err),
            JingleError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            JingleError::PlaybackError(msg) => write!(f, "Audio playback error: {}", msg),
        }
    }
}

impl std::error::Error for JingleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JingleError::IoError(err) => Some(err),
            JingleError::WavError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for JingleError {
    fn from(err: std::io::Error) -> Self {
        JingleError::IoError(err)
    }
}

impl From<hound::Error> for JingleError {
    fn from(err: hound::Error) -> Self {
        JingleError::WavError(err)
    }
}

/// Result type alias for jingle operations
pub type Result<T> = std::result::Result<T, JingleError>;