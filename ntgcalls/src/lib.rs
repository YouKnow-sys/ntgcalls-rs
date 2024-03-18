use std::{
    mem::{transmute, MaybeUninit},
    ptr,
    sync::Arc,
};

use libntgcalls_sys::{
    ntg_calls, ntg_calls_count, ntg_change_stream, ntg_connect, ntg_destroy, ntg_get_params,
    ntg_get_state, ntg_get_version, ntg_init, ntg_media_description_struct, ntg_mute, ntg_pause,
    ntg_resume, ntg_stop, ntg_time, ntg_unmute, NTG_ERR_TOO_SMALL,
};

use errors::{DestroyError, NTgCallError, NTgCallResult};
use structures::{GroupCall, MediaDescription, MediaState};
use utils::IntoCString;

use crate::structures::{AudioDescription, VideoDescription};

pub mod enums;
pub mod errors;
pub mod structures;
pub mod utils;

struct NTgCallInner(u32);

/// NTgCall is a wrapper struct that encapsulates an instance of
/// the NTgCalls API. It contains the identifier UID for the NTgCalls
/// instance.
#[derive(Clone)]
pub struct NTgCall {
    inner: Arc<NTgCallInner>,
}

/// Initialization and De-Initialization
impl NTgCall {
    /// Initialize a new NTgCall instance.
    ///
    /// ## Note
    /// If you need multiple instance of [`NTgCall`] for multiple threads its better
    /// to create all the instance in your main thread and then just pass them to other threads.
    ///
    /// ## Return
    /// A new instance of the [`NTgCall`]
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(NTgCallInner(unsafe { ntg_init() })),
        }
    }

    /// Clean up and release the resources used by NTgCalls
    pub fn destroy(self) -> Result<(), DestroyError> {
        let result = unsafe { ntg_destroy(self.inner.0) };

        if result != 0 {
            return Err(DestroyError);
        }

        Ok(())
    }

    /// Retrieve the version of NTgCalls library.
    pub fn version() -> String {
        let mut buf = [0u8; 8];

        let result = unsafe { ntg_get_version(transmute(buf.as_mut_ptr()), buf.len() as _) };

        assert_ne!(
            result, NTG_ERR_TOO_SMALL,
            "Version buffer is too small, NTgCalls expected a bigger buffer, this is a internal error, report it!"
        );

        String::from_utf8_lossy(&buf).into_owned()
    }
}

/// Basic methods
impl NTgCall {
    /// This method allows connecting to Telegram Group Calls with the output params from Telegram method [`JoinGroupCall`].
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    /// - `params`: Connection params obtained from Telegram.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case of failure.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::RtmpNeeded`]
    /// - [`NTgCallError::InvalidTransport`]
    /// - [`NTgCallError::ConnectionFailed`]
    /// - [`NTgCallError::UnknownException`]
    ///
    /// [JoinGroupCall]: https://core.telegram.org/method/phone.joinGroupCall
    pub fn connect<S: IntoCString>(&self, chat_id: i64, params: S) -> NTgCallResult<()> {
        let params = params.into_c_string();

        let result = unsafe { ntg_connect(self.inner.0, chat_id, params.into_raw()) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(())
    }

    /// This method allows getting connection params for Telegram WebRTC connection.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    /// - `desc`: Media parameters of the stream.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and the `params` in case of success.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionAlreadyExists`]
    /// - [`NTgCallError::FileNotFound`]
    /// - [`NTgCallError::EncoderNotFound`]
    /// - [`NTgCallError::FFmpegNotFound`]
    /// - [`NTgCallError::ShellError`]
    /// - [`NTgCallError::UnknownException`]
    pub fn get_params(&self, chat_id: i64, desc: MediaDescription) -> NTgCallResult<String> {
        let mut buf = vec![0; 512];

        let audio = desc.audio.as_ref().map(AudioDescription::to_ffi);
        let video = desc.video.as_ref().map(VideoDescription::to_ffi);

        let ffi_desc = ntg_media_description_struct {
            audio: audio.map_or(ptr::null(), |n| &n as *const _),
            video: video.map_or(ptr::null(), |n| &n as *const _),
        };

        let result = unsafe {
            ntg_get_params(
                self.inner.0,
                chat_id,
                ffi_desc,
                transmute(buf.as_mut_ptr()),
                512,
            )
        };

        assert_ne!(
            result, NTG_ERR_TOO_SMALL,
            "Params buffer is too small, NTgCalls expected a bigger buffer, this is a internal error, report it!"
        );

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(String::from_utf8(buf).expect(
            "GetParams function returned invalid string. this is a internal error, report it",
        ))
    }

    /// This method allow stopping a WebRTC connection.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case of failure.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    pub fn stop(&self, chat_id: i64) -> NTgCallResult<()> {
        let result = unsafe { ntg_stop(self.inner.0, chat_id) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(())
    }
}

/// Stream methods
impl NTgCall {
    /// This method allows changing the streaming file in a Group Call.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    /// - `desc`: Media parameters of the stream.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionAlreadyExists`]
    /// - [`NTgCallError::FileNotFound`]
    /// - [`NTgCallError::EncoderNotFound`]
    /// - [`NTgCallError::FFmpegNotFound`]
    /// - [`NTgCallError::ShellError`]
    /// - [`NTgCallError::UnknownException`]
    pub fn change_stream(&self, chat_id: i64, desc: MediaDescription) -> NTgCallResult<()> {
        let audio = desc.audio.as_ref().map(AudioDescription::to_ffi);
        let video = desc.video.as_ref().map(VideoDescription::to_ffi);

        let ffi_desc = ntg_media_description_struct {
            audio: audio.map_or(ptr::null(), |n| &n as *const _),
            video: video.map_or(ptr::null(), |n| &n as *const _),
        };

        let result = unsafe { ntg_change_stream(self.inner.0, chat_id, ffi_desc) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(())
    }

    /// This method allows you to mute the WebRTC stream.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a [`bool`] in case of success, if
    /// the return value is `true` it mean that the audio is now muted else the
    /// audio was already muted.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    pub fn mute(&self, chat_id: i64) -> NTgCallResult<bool> {
        let result = unsafe { ntg_mute(self.inner.0, chat_id) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(result == 0)
    }

    /// This method allows you to pause the WebRTC stream.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a [`bool`] in case of success, if
    /// the return value is `true` it mean that the audio is now paused else the
    /// audio was already paused.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    pub fn pause(&self, chat_id: i64) -> NTgCallResult<bool> {
        let result = unsafe { ntg_pause(self.inner.0, chat_id) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(result == 0)
    }

    /// This method allows you to resume the WebRTC stream.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a [`bool`] in case of success, if
    /// the return value is `true` it mean that the audio is now resumed else the
    /// audio was not paused.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    pub fn resume(&self, chat_id: i64) -> NTgCallResult<bool> {
        let result = unsafe { ntg_resume(self.inner.0, chat_id) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(result == 0)
    }

    /// This method allows getting the played time of the stream.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and the played time in case of success.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    pub fn played_time(&self, chat_id: i64) -> NTgCallResult<i64> {
        let result = unsafe { ntg_time(self.inner.0, chat_id) };

        if result.is_negative() {
            return Err(NTgCallError::from(result as i32));
        }

        Ok(result)
    }

    /// This method allows you to mute the WebRTC stream.
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a [`bool`] in case of success, if
    /// the return value is `true` it mean that the audio is now unmuted else the
    /// audio was not muted.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    pub fn unmute(&self, chat_id: i64) -> NTgCallResult<bool> {
        let result = unsafe { ntg_unmute(self.inner.0, chat_id) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(result == 0)
    }
}

/// Advance methods
impl NTgCall {
    /// This function returns the number of active group calls that NTgCalls is connected to.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a [`i32`] showing number active groups of in case of success.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::UnknownException`]
    pub fn count_calls(&self) -> NTgCallResult<i32> {
        let result = unsafe { ntg_calls_count(self.inner.0) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        Ok(result)
    }

    /// This function returns a list of [`GroupCall`] instances, each containing information
    /// about an active group call associated with the channel or group.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a list of [`GroupCall`] in case of success.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::UnknownException`]
    pub fn calls(&self) -> NTgCallResult<Vec<GroupCall>> {
        let count = self.count_calls()?;

        let mut buffer = Vec::with_capacity(count as usize);

        let result = unsafe { ntg_calls(self.inner.0, buffer.as_mut_ptr(), count) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        unsafe {
            buffer.set_len(count as usize);
            // this should be ok because both ntg_group_call_start and group call have the same layout...
            Ok(transmute(buffer))
        }
    }

    /// This method allows retrieving GroupCall MTProto Configurations for [`EditGroupCallParticipant`].
    ///
    ///
    /// ## Parameters
    /// - `chat_id`: Unique identifier of a chat.
    ///
    /// ## Return
    /// A [`NTgCallError`] in case failure and a [`MediaState`] in case of success that store all
    /// the configurations.
    ///
    /// ### Possible error values
    /// - [`NTgCallError::InvalidUid`]
    /// - [`NTgCallError::ConnectionNotFound`]
    /// - [`NTgCallError::UnknownException`]
    ///
    /// [`EditGroupCallParticipant`]: https://core.telegram.org/method/phone.editGroupCallParticipant
    pub fn get_state(&self, chat_id: i64) -> NTgCallResult<MediaState> {
        let mut buffer = MaybeUninit::uninit();

        let result = unsafe { ntg_get_state(self.inner.0, chat_id, buffer.as_mut_ptr()) };

        if result.is_negative() {
            return Err(NTgCallError::from(result));
        }

        let state = unsafe { buffer.assume_init() };

        Ok(MediaState {
            muted: state.muted,
            video_paused: state.videoPaused,
            video_stopped: state.videoStopped,
        })
    }
}

impl Drop for NTgCallInner {
    // Clean up and release all the resource allocated by the instance.
    fn drop(&mut self) {
        // if this ever fail we leak memory...
        let _ = unsafe { ntg_destroy(self.0) };
    }
}

#[cfg(test)]
mod test {
    use crate::{structures::MediaDescription, NTgCall};

    #[test]
    fn test_all_sequential() {
        binding_working();
        clone();
    }

    /// A test to make sure the binding works
    fn binding_working() {
        let call0 = NTgCall::new();
        let call1 = NTgCall::new();
        let call2 = NTgCall::new();
        let version = NTgCall::version();

        assert_eq!(call0.inner.0, 0);
        assert_eq!(call1.inner.0, 1);
        assert_eq!(call2.inner.0, 2);

        call0.destroy().unwrap();
        call1.destroy().unwrap();
        call2.destroy().unwrap();

        assert!(
            !version.is_empty(),
            "Version can't be empty, there was a problem in binding"
        );
    }

    fn clone() {
        let call = NTgCall::new();

        let call2 = call.clone();
        let call3 = call.clone();

        // drop one of instance, other instance should still be valid
        drop(call);

        // check if the instance is still valid
        let params = call2.get_params(123, MediaDescription::default()).unwrap();
        assert!(!params.is_empty());

        call3.destroy().unwrap();

        assert!(call2.destroy().is_err());
    }
}
