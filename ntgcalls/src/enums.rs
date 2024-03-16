/// Enumeration of input modes for audio and video streams.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum InputMode {
    /// Input mode using a file source.
    File = 1,
    /// Input mode using a shell command.
    Shell = 2,
    /// Input mode using FFmpeg.
    FFmpeg = 4,
    /// Exprimental mode
    NoLatency = 8,
}

/// Enumeration of stream status values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum StreamStatus {
    /// Indicates that the stream is currently playing.
    Playing = 0,
    /// Indicates that the stream is currently paused.
    Paused = 1,
    /// Indicates that the stream is currently idling (neither playing nor paused).
    Idling = 2,
}

/// Enumeration of stream type values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum StreamType {
    /// Indicates that the stream type is audio.
    Audio = 0,
    /// Indicates that the stream type is video.
    Video = 1,
}
