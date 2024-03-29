/* automatically generated by rust-bindgen 0.69.4 */

pub const NTG_CONNECTION_ALREADY_EXISTS: i32 = -100;
pub const NTG_CONNECTION_NOT_FOUND: i32 = -101;
pub const NTG_FILE_NOT_FOUND: i32 = -200;
pub const NTG_ENCODER_NOT_FOUND: i32 = -201;
pub const NTG_FFMPEG_NOT_FOUND: i32 = -202;
pub const NTG_SHELL_ERROR: i32 = -203;
pub const NTG_RTMP_NEEDED: i32 = -300;
pub const NTG_INVALID_TRANSPORT: i32 = -301;
pub const NTG_CONNECTION_FAILED: i32 = -302;
pub const NTG_UNKNOWN_EXCEPTION: i32 = -1;
pub const NTG_INVALID_UID: i32 = -2;
pub const NTG_ERR_TOO_SMALL: i32 = -3;
pub const ntg_input_mode_enum_NTG_FILE: ntg_input_mode_enum = 1;
pub const ntg_input_mode_enum_NTG_SHELL: ntg_input_mode_enum = 2;
pub const ntg_input_mode_enum_NTG_FFMPEG: ntg_input_mode_enum = 4;
pub const ntg_input_mode_enum_NTG_NO_LATENCY: ntg_input_mode_enum = 8;
pub type ntg_input_mode_enum = ::std::os::raw::c_int;
pub const ntg_stream_type_enum_NTG_STREAM_AUDIO: ntg_stream_type_enum = 0;
pub const ntg_stream_type_enum_NTG_STREAM_VIDEO: ntg_stream_type_enum = 1;
pub type ntg_stream_type_enum = ::std::os::raw::c_int;
pub const ntg_stream_status_enum_NTG_PLAYING: ntg_stream_status_enum = 0;
pub const ntg_stream_status_enum_NTG_PAUSED: ntg_stream_status_enum = 1;
pub const ntg_stream_status_enum_NTG_IDLING: ntg_stream_status_enum = 2;
pub type ntg_stream_status_enum = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ntg_audio_description_struct {
    pub inputMode: ntg_input_mode_enum,
    pub input: *const ::std::os::raw::c_char,
    pub sampleRate: u32,
    pub bitsPerSample: u8,
    pub channelCount: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ntg_video_description_struct {
    pub inputMode: ntg_input_mode_enum,
    pub input: *const ::std::os::raw::c_char,
    pub width: u16,
    pub height: u16,
    pub fps: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ntg_media_description_struct {
    pub audio: *const ntg_audio_description_struct,
    pub video: *const ntg_video_description_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ntg_group_call_struct {
    pub chatId: i64,
    pub status: ntg_stream_status_enum,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ntg_media_state_struct {
    pub muted: bool,
    pub videoPaused: bool,
    pub videoStopped: bool,
}

pub type ntg_stream_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: u32, arg2: i64, arg3: ntg_stream_type_enum)>;
pub type ntg_upgrade_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: u32, arg2: i64, arg3: ntg_media_state_struct)>;
pub type ntg_disconnect_callback =
    ::std::option::Option<unsafe extern "C" fn(arg1: u32, arg2: i64)>;

extern "C" {
    #[must_use]
    pub fn ntg_init() -> u32;

    pub fn ntg_destroy(uid: u32) -> ::std::os::raw::c_int;

    #[must_use]
    pub fn ntg_get_params(
        uid: u32,
        chatID: i64,
        desc: ntg_media_description_struct,
        buffer: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn ntg_connect(
        uid: u32,
        chatID: i64,
        params: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn ntg_change_stream(
        uid: u32,
        chatID: i64,
        desc: ntg_media_description_struct,
    ) -> ::std::os::raw::c_int;

    pub fn ntg_pause(uid: u32, chatID: i64) -> ::std::os::raw::c_int;

    pub fn ntg_resume(uid: u32, chatID: i64) -> ::std::os::raw::c_int;

    pub fn ntg_mute(uid: u32, chatID: i64) -> ::std::os::raw::c_int;

    pub fn ntg_unmute(uid: u32, chatID: i64) -> ::std::os::raw::c_int;

    pub fn ntg_stop(uid: u32, chatID: i64) -> ::std::os::raw::c_int;

    pub fn ntg_time(uid: u32, chatID: i64) -> i64;

    pub fn ntg_get_state(
        uid: u32,
        chatID: i64,
        mediaState: *mut ntg_media_state_struct,
    ) -> ::std::os::raw::c_int;

    pub fn ntg_calls(
        uid: u32,
        buffer: *mut ntg_group_call_struct,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[must_use]
    pub fn ntg_calls_count(uid: u32) -> ::std::os::raw::c_int;

    pub fn ntg_on_stream_end(uid: u32, callback: ntg_stream_callback) -> ::std::os::raw::c_int;

    pub fn ntg_on_upgrade(uid: u32, callback: ntg_upgrade_callback) -> ::std::os::raw::c_int;

    pub fn ntg_on_disconnect(uid: u32, callback: ntg_disconnect_callback) -> ::std::os::raw::c_int;

    pub fn ntg_get_version(
        buffer: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn ntg_cpu_usage(uid: u32, buffer: *mut f64) -> ::std::os::raw::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindgen_test_layout_ntg_audio_description_struct() {
        const UNINIT: ::std::mem::MaybeUninit<ntg_audio_description_struct> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ntg_audio_description_struct>(),
            24usize,
            concat!("Size of: ", stringify!(ntg_audio_description_struct))
        );
        assert_eq!(
            ::std::mem::align_of::<ntg_audio_description_struct>(),
            8usize,
            concat!("Alignment of ", stringify!(ntg_audio_description_struct))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).inputMode) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_audio_description_struct),
                "::",
                stringify!(inputMode)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).input) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_audio_description_struct),
                "::",
                stringify!(input)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).sampleRate) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_audio_description_struct),
                "::",
                stringify!(sampleRate)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).bitsPerSample) as usize - ptr as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_audio_description_struct),
                "::",
                stringify!(bitsPerSample)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).channelCount) as usize - ptr as usize },
            21usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_audio_description_struct),
                "::",
                stringify!(channelCount)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_ntg_video_description_struct() {
        const UNINIT: ::std::mem::MaybeUninit<ntg_video_description_struct> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ntg_video_description_struct>(),
            24usize,
            concat!("Size of: ", stringify!(ntg_video_description_struct))
        );
        assert_eq!(
            ::std::mem::align_of::<ntg_video_description_struct>(),
            8usize,
            concat!("Alignment of ", stringify!(ntg_video_description_struct))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).inputMode) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_video_description_struct),
                "::",
                stringify!(inputMode)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).input) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_video_description_struct),
                "::",
                stringify!(input)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_video_description_struct),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
            18usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_video_description_struct),
                "::",
                stringify!(height)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).fps) as usize - ptr as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_video_description_struct),
                "::",
                stringify!(fps)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_ntg_media_description_struct() {
        const UNINIT: ::std::mem::MaybeUninit<ntg_media_description_struct> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ntg_media_description_struct>(),
            16usize,
            concat!("Size of: ", stringify!(ntg_media_description_struct))
        );
        assert_eq!(
            ::std::mem::align_of::<ntg_media_description_struct>(),
            8usize,
            concat!("Alignment of ", stringify!(ntg_media_description_struct))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).audio) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_media_description_struct),
                "::",
                stringify!(audio)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).video) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_media_description_struct),
                "::",
                stringify!(video)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_ntg_group_call_struct() {
        const UNINIT: ::std::mem::MaybeUninit<ntg_group_call_struct> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ntg_group_call_struct>(),
            16usize,
            concat!("Size of: ", stringify!(ntg_group_call_struct))
        );
        assert_eq!(
            ::std::mem::align_of::<ntg_group_call_struct>(),
            8usize,
            concat!("Alignment of ", stringify!(ntg_group_call_struct))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).chatId) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_group_call_struct),
                "::",
                stringify!(chatId)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_group_call_struct),
                "::",
                stringify!(status)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_ntg_media_state_struct() {
        const UNINIT: ::std::mem::MaybeUninit<ntg_media_state_struct> =
            ::std::mem::MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::std::mem::size_of::<ntg_media_state_struct>(),
            3usize,
            concat!("Size of: ", stringify!(ntg_media_state_struct))
        );
        assert_eq!(
            ::std::mem::align_of::<ntg_media_state_struct>(),
            1usize,
            concat!("Alignment of ", stringify!(ntg_media_state_struct))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).muted) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_media_state_struct),
                "::",
                stringify!(muted)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).videoPaused) as usize - ptr as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_media_state_struct),
                "::",
                stringify!(videoPaused)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).videoStopped) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(ntg_media_state_struct),
                "::",
                stringify!(videoStopped)
            )
        );
    }
}
