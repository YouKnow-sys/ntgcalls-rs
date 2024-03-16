use std::{ffi::CString, ptr};

use libntgcalls_sys::{
    ntg_audio_description_struct, ntg_input_mode_enum, ntg_media_description_struct,
    ntg_video_description_struct,
};

use crate::{
    enums::{InputMode, StreamStatus},
    utils::IntoCString,
};

/// Media Configuration for the Stream.
#[derive(Debug, Clone)]
pub struct MediaDescription {
    /// Audio configuration for the stream.
    pub audio: Option<AudioDescription>,
    /// Video configuration for the stream.
    pub video: Option<VideoDescription>,
}

impl MediaDescription {
    pub(crate) fn to_ffi(&self) -> ntg_media_description_struct {
        let audio = self
            .audio
            .as_ref()
            .map_or(ptr::null(), |r| &r.to_ffi() as *const _);

        let video = self
            .video
            .as_ref()
            .map_or(ptr::null(), |r| &r.to_ffi() as *const _);

        ntg_media_description_struct { audio, video }
    }
}

/// Stream’s Audio Configuration
#[derive(Debug, Clone)]
pub struct AudioDescription {
    /// The input mode for audio.
    pub input_mode: InputMode,
    /// The input media source.
    input: CString,
    /// Audio sample rate (0-96000, max allowed by PCM16L).
    pub sample_rate: u32,
    /// Audio bits per sample (8 or 16).
    pub bits_per_sample: u8,
    /// Audio channel count (1-2, max allowed by PCM16L).
    pub channel_count: u8,
}

impl AudioDescription {
    /// Create a new [`AudioDescription`]
    pub fn new<S: IntoCString>(
        input_mode: InputMode,
        input: S,
        sample_rate: u32,
        bits_per_sample: u8,
        channel_count: u8,
    ) -> Self {
        Self {
            input_mode,
            input: input.into_c_string(),
            sample_rate,
            bits_per_sample,
            channel_count,
        }
    }

    pub(crate) fn to_ffi(&self) -> ntg_audio_description_struct {
        ntg_audio_description_struct {
            inputMode: self.input_mode as ntg_input_mode_enum,
            input: self.input.as_ptr(),
            sampleRate: self.sample_rate,
            bitsPerSample: self.bits_per_sample,
            channelCount: self.channel_count,
        }
    }
}

/// Stream’s Video Configuration
#[derive(Debug, Clone)]
pub struct VideoDescription {
    /// The input mode for video.
    pub input_mode: InputMode,
    /// The input media source.
    input: CString,
    /// Video width in pixels.
    pub width: u16,
    /// Video height in pixels.
    pub height: u16,
    /// Frames per second (FPS) for video playback.
    pub fps: u8,
}

impl VideoDescription {
    /// Create a new [`VideoDescription`]
    pub fn new<S: IntoCString>(
        input_mode: InputMode,
        input: S,
        width: u16,
        height: u16,
        fps: u8,
    ) -> Self {
        Self {
            input_mode,
            input: input.into_c_string(),
            width,
            height,
            fps,
        }
    }

    pub(crate) fn to_ffi(&self) -> ntg_video_description_struct {
        ntg_video_description_struct {
            inputMode: self.input_mode as ntg_input_mode_enum,
            input: self.input.as_ptr(),
            width: self.width,
            height: self.height,
            fps: self.fps,
        }
    }
}

/// Info about a group call
#[derive(Debug, Clone)]
#[repr(C)]
pub struct GroupCall {
    /// Unique identifier of a chat.
    pub chat_id: i64,
    /// Stream’s Status.
    pub stream_status: StreamStatus,
}

/// GroupCall MTProto Configurations.
#[derive(Debug, Clone)]
pub struct MediaState {
    /// Indicates whether the media stream is muted.
    pub muted: bool,
    /// Indicates whether the video stream is paused.
    pub video_paused: bool,
    /// Indicates whether the video stream is stopped.
    pub video_stopped: bool,
}

#[cfg(test)]
mod test {
    use libntgcalls_sys::{ntg_group_call_struct, ntg_stream_status_enum};

    use super::*;
    use std::mem::size_of;

    #[test]
    fn group_call_is_same_as_ntg_group_call_struct() {
        assert_eq!(size_of::<GroupCall>(), size_of::<ntg_group_call_struct>());
        assert_eq!(
            size_of::<StreamStatus>(),
            size_of::<ntg_stream_status_enum>()
        );
    }
}
