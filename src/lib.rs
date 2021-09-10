#![feature(const_raw_ptr_deref)]

use std::ffi::CStr;

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        $crate::cstr_from_bytes(concat!($s, '\0').as_bytes())
    };
}

pub const fn cstr_from_bytes(bytes: &[u8]) -> &CStr {
    unsafe { &*(bytes as *const [u8] as *const CStr) }
}

#[test]
fn test() {
    const TEST_STRING: &'static str = "It's a\ntest string\tyay";
    const TEST_STRING_C: &'static CStr = cstr!("It's a\ntest string\tyay");

    assert!(TEST_STRING == TEST_STRING_C.to_str().unwrap());
}
