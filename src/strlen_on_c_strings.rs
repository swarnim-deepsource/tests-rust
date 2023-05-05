#[rustfmt::skip]
mod test {
    use std::ffi::{CStr, CString};
    fn trivial() {
        // CString
        let cstring = CString::new("foo").expect("CString::new failed");
                let _ = unsafe { libc::strlen(cstring.as_ptr()) };

        // CStr
        let cstr = CStr::from_bytes_with_nul(b"foo\0").expect("CStr::from_bytes_with_nul failed");
                let _ = unsafe { libc::strlen(cstr.as_ptr()) };

        let pcstr: *const &CStr = &cstr;
                let _ = unsafe { libc::strlen((*pcstr).as_ptr()) };
    }
}
