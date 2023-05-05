#[rustfmt::skip]
mod test {
    use std::ffi::{CStr, CString};
    fn trivial() {
        // CString
        let cstring = CString::new("foo").expect("CString::new failed");
        //> [RS-W1098]: "Calling `libc::strlen` upon a `CString`, use `.as_bytes().len()` instead"
        let _ = unsafe { libc::strlen(cstring.as_ptr()) };

        // CStr
        let cstr = CStr::from_bytes_with_nul(b"foo\0").expect("CStr::from_bytes_with_nul failed");
        //> [RS-W1098]: "Calling `libc::strlen` upon a `CStr`, use `.to_bytes().len()` instead"
        let _ = unsafe { libc::strlen(cstr.as_ptr()) };

        let pcstr: *const &CStr = &cstr;
        //> [RS-W1098]: "Calling `libc::strlen` upon a `CStr`, use `.to_bytes().len()` instead"
        let _ = unsafe { libc::strlen((*pcstr).as_ptr()) };
    }
}
