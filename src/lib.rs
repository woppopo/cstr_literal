#![feature(const_cstr_unchecked)]

use std::ffi::CStr;

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        $crate::_cstr_from_bytes(concat!($s, '\0').as_bytes())
    };
}

#[doc(hidden)]
pub const fn _cstr_from_bytes(bytes: &[u8]) -> &CStr {
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(bytes) }
}

#[test]
fn test() {
    const TEST_STRING: &'static str = "It's a\ntest string\tyay";
    const TEST_STRING_C: &'static CStr = cstr!("It's a\ntest string\tyay");

    assert!(TEST_STRING == TEST_STRING_C.to_str().unwrap());
}
