use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn great() -> *mut c_char {
    CString::new("hejsan").unwrap().into_raw()
}


// http://webassembly.github.io/spec/core/appendix/index-types.html

/*
function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}
*/  