use std::ffi::{CStr, CString};

/// A helper trait to convert multiple strings type to CString to be used with ntg library
pub trait IntoCString {
    /// convert the string to CString, this operation shouldn't fail.
    fn into_c_string(self) -> CString;
}

impl IntoCString for String {
    fn into_c_string(self) -> CString {
        match CString::new(self) {
            Ok(s) => s,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        }
    }
}

impl IntoCString for &str {
    fn into_c_string(self) -> CString {
        match CString::new(self) {
            Ok(s) => s,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        }
    }
}

impl IntoCString for CString {
    fn into_c_string(self) -> CString {
        self
    }
}

impl IntoCString for &CStr {
    fn into_c_string(self) -> CString {
        CString::from(self)
    }
}
