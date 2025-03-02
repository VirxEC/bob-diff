use core::{ffi::CStr, slice};
use std::path::PathBuf;

mod diff;

fn cstr_to_path(cstr: *const std::os::raw::c_char) -> Option<PathBuf> {
    let cstr = unsafe { CStr::from_ptr(cstr) };
    Some(PathBuf::from(cstr.to_str().ok()?))
}

fn vec_to_buf(vec: Vec<u8>) -> (*mut std::os::raw::c_char, u32) {
    let len = vec.len() as u32;
    let buf = vec.as_ptr();

    (buf as *mut std::os::raw::c_char, len)
}

fn buf_to_slice(buf: *const std::os::raw::c_char, len: u32) -> &'static [u8] {
    let buf = buf as *const u8;
    let len = len as usize;

    unsafe { slice::from_raw_parts(buf, len) }
}

/// # Safety
///
/// This function is unsafe because it dereferences raw pointers and assumes that the data they point to is valid.
///
/// The caller must ensure that the pointers are valid and that the data they point to is valid for the duration of the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn diff(
    old: *const std::os::raw::c_char,
    new: *const std::os::raw::c_char,
    out_buf: *mut *mut std::os::raw::c_char,
    out_buf_len: *mut u32,
) -> u16 {
    let Some(old) = cstr_to_path(old) else {
        return 1;
    };

    let Some(new) = cstr_to_path(new) else {
        return 1;
    };

    match diff::command_diff(old, new) {
        Ok(buf) => {
            let (buf, len) = vec_to_buf(buf);

            unsafe {
                *out_buf = buf;
                *out_buf_len = len;
            }

            0
        }
        Err(e) => {
            eprintln!("Error: {e:?}");
            1
        }
    }
}

/// # Safety
///
/// This function is unsafe because it dereferences raw pointers and assumes that the data they point to is valid.
///
/// The caller must ensure that the pointers are valid and that the data they point to is valid for the duration of the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn diff_apply(
    dir: *const std::os::raw::c_char,
    buf: *const std::os::raw::c_char,
    buf_len: u32,
) -> u16 {
    let Some(dir) = cstr_to_path(dir) else {
        return 1;
    };

    let buf = buf_to_slice(buf, buf_len);

    if let Err(e) = diff::command_diff_apply(dir, buf) {
        eprintln!("Error: {e:?}");
        1
    } else {
        0
    }
}
