use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub fn path_to_cstring(path: &Path) -> CString {
    CString::new(path.as_os_str().as_bytes()).unwrap()
}

pub fn as_path(cstring: &CStr) -> &Path {
    OsStr::from_bytes(cstring.to_bytes()).as_ref()
}
