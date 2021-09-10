#![feature(const_cstr_unchecked)]

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, '\0').as_bytes()) }
    };
}

#[test]
fn test() {
    use std::ffi::CStr;

    const TEST_STRING: &'static str = "It's a\ntest string\tyay";
    const TEST_STRING_C: &'static CStr = cstr!("It's a\ntest string\tyay");

    assert!(TEST_STRING == TEST_STRING_C.to_str().unwrap());
}
