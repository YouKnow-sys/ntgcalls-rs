use std::{error::Error, fmt::Display};

use libntgcalls_sys::{
    NTG_CONNECTION_ALREADY_EXISTS, NTG_CONNECTION_FAILED, NTG_CONNECTION_NOT_FOUND,
    NTG_ENCODER_NOT_FOUND, NTG_FFMPEG_NOT_FOUND, NTG_FILE_NOT_FOUND, NTG_INVALID_TRANSPORT,
    NTG_INVALID_UID, NTG_RTMP_NEEDED, NTG_SHELL_ERROR,
};

/// Result type alias for NTgCall errors.
pub type NTgCallResult<T> = Result<T, NTgCallError>;

/// All errors related to NTgCalls
#[derive(Debug)]
#[repr(i32)]
pub enum NTgCallError {
    /// A connection with the specified ID already exists.
    ConnectionAlreadyExists,
    /// The specified connection was not found.
    ConnectionNotFound,
    /// The specified file was not found.
    FileNotFound,
    /// The required encoder was not found.
    EncoderNotFound,
    /// FFmpeg is not found in the system.
    FFmpegNotFound,
    /// An error occurred while executing a shell command.
    ShellError,
    /// This error occurs when attempting to join a group call that requires an RTMP transport.
    RtmpNeeded,
    /// The specified transport is invalid.
    InvalidTransport,
    /// The connection to WebRTC failed.
    ConnectionFailed,
    /// An unknown exception occurred.
    UnknownException,
    /// The provided UID is invalid.
    InvalidUid,
    // ErrTooSmall -> we handle this as a internal error
}

impl Error for NTgCallError {}

impl Display for NTgCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionAlreadyExists => "[ConnectionAlreadyExists]: A connection with the specified ID already exists",
            Self::ConnectionNotFound => "[ConnectionNotFound]: The specified connection was not found",
            Self::FileNotFound => "[FileNotFound]: The specified file was not found",
            Self::EncoderNotFound => "[EncoderNotFound]: The required encoder was not found",
            Self::FFmpegNotFound => "[FfmpegNotFound]: FFmpeg is not found in the system",
            Self::ShellError => "[ShellError]: An error occurred while executing a shell command",
            Self::RtmpNeeded => "[RtmpNeeded]: This error occurs when attempting to join a group call that requires an RTMP transport",
            Self::InvalidTransport => "[InvalidTransport]: The specified transport is invalid",
            Self::ConnectionFailed => "[ConnectionFailed]: The connection to WebRTC failed",
            Self::UnknownException => "[UnknownException]: An unknown exception occurred",
            Self::InvalidUid => "[InvalidUid]: The provided UID is invalid",
        }
        .fmt(f)
    }
}

impl From<i32> for NTgCallError {
    fn from(value: i32) -> Self {
        match value {
            NTG_CONNECTION_ALREADY_EXISTS => Self::ConnectionAlreadyExists,
            NTG_CONNECTION_NOT_FOUND => Self::ConnectionNotFound,
            NTG_FILE_NOT_FOUND => Self::FileNotFound,
            NTG_ENCODER_NOT_FOUND => Self::EncoderNotFound,
            NTG_FFMPEG_NOT_FOUND => Self::FFmpegNotFound,
            NTG_SHELL_ERROR => Self::ShellError,
            NTG_RTMP_NEEDED => Self::RtmpNeeded,
            NTG_INVALID_TRANSPORT => Self::InvalidTransport,
            NTG_CONNECTION_FAILED => Self::ConnectionFailed,
            NTG_INVALID_UID => Self::InvalidUid,
            _ => Self::UnknownException,
        }
    }
}

/// DestroyError is a struct representing an error that occurs during destruction.
/// This is typically used to represent errors freeing resources during drop.
#[derive(Debug)]
pub struct DestroyError;

impl Error for DestroyError {}

impl Display for DestroyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "[DestroyError]: NTgCalls cleanup failed".fmt(f)
    }
}
