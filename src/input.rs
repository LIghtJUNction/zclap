// The private module seals the supported representations. This is plumbing,
// not an extension point: callers use str, bytes, or native OS strings directly.
pub trait Input {
    type Value: ?Sized;
    fn bytes(&self) -> &[u8];
    fn suffix(&self, start: usize) -> &Self::Value;
}

impl Input for &str {
    type Value = str;
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn suffix(&self, start: usize) -> &str {
        &self[start..]
    }
}

impl Input for &[u8] {
    type Value = [u8];
    fn bytes(&self) -> &[u8] {
        self
    }
    fn suffix(&self, start: usize) -> &[u8] {
        &self[start..]
    }
}

#[cfg(feature = "std")]
impl Input for std::string::String {
    type Value = str;
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn suffix(&self, start: usize) -> &str {
        &self[start..]
    }
}

#[cfg(feature = "std")]
impl Input for std::ffi::OsString {
    type Value = std::ffi::OsStr;
    fn bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }
    fn suffix(&self, start: usize) -> &std::ffi::OsStr {
        os_suffix(self, start)
    }
}

#[cfg(feature = "std")]
impl Input for &std::ffi::OsStr {
    type Value = std::ffi::OsStr;
    fn bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }
    fn suffix(&self, start: usize) -> &std::ffi::OsStr {
        os_suffix(self, start)
    }
}

#[cfg(feature = "std")]
#[allow(unsafe_code)]
fn os_suffix(value: &std::ffi::OsStr, start: usize) -> &std::ffi::OsStr {
    let bytes = value.as_encoded_bytes();
    // SAFETY: bytes originate from this OsStr, in the same process/platform.
    // The parser passes either 0 (whole token) or the end of a valid UTF-8
    // option prefix: --NAME=, -SHORT, or -SHORT=. Unicode short names are
    // decoded before their boundary is used. Thus a nonzero cut immediately
    // follows a valid nonempty UTF-8 substring, as required by OsStr's contract.
    // No arbitrary byte-input path can call this function.
    debug_assert!(start == 0 || core::str::from_utf8(&bytes[..start]).is_ok());
    unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(&bytes[start..]) }
}
