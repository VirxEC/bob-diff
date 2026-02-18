use core::{
    ffi::{CStr, c_char},
    slice,
};
use std::{mem, path::PathBuf};

mod diff;

fn cstr_to_path(cstr: *const c_char) -> Option<PathBuf> {
    let cstr = unsafe { CStr::from_ptr(cstr) };
    Some(PathBuf::from(cstr.to_str().ok()?))
}

fn vec_to_buf(mut buf: Vec<u8>) -> (*mut c_char, u64) {
    buf.shrink_to_fit();
    let mut buf = mem::ManuallyDrop::new(buf);

    (buf.as_mut_ptr().cast::<c_char>(), buf.len() as u64)
}

const fn buf_to_slice(buf: *const c_char, len: usize) -> &'static [u8] {
    unsafe { slice::from_raw_parts(buf.cast::<u8>(), len) }
}

/// # Safety
///
/// This function is unsafe because it dereferences raw pointers and assumes that they point to null-terminated strings.
///
/// The caller must ensure that the pointers `old` and `new` are valid and that the data they point to is valid for the duration of the call.
/// If `old` or `new` is simply not valid UTF-8, the error code 1 will be returned.
///
/// `out_buf` and `out_buf_len` are never read, only written to - they do not have to point to valid data.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn diff(
    old: *const c_char,
    new: *const c_char,
    out_buf: *mut *mut c_char,
    out_buf_len: *mut u64,
) -> u16 {
    let Some(old) = cstr_to_path(old) else {
        eprintln!("'old' did not contain valid utf-8");
        return 1;
    };

    let Some(new) = cstr_to_path(new) else {
        eprintln!("'new' did not contain valid utf-8");
        return 1;
    };

    match diff::command_diff(&old, &new) {
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
/// This function is unsafe because it dereferences a raw pointer and assumes that it points to a null-terminated string.
///
/// The caller must ensure that the pointer `dir` is valid and that the data it points to is valid for the duration of the call.
/// If `dir` is simply not valid UTF-8, the error code 1 will be returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn diff_apply(dir: *const c_char, buf: *const c_char, buf_len: u64) -> u16 {
    let Some(dir) = cstr_to_path(dir) else {
        eprintln!("'dir' did not contain valid utf-8");
        return 1;
    };

    let Ok(len) = usize::try_from(buf_len) else {
        return 1;
    };

    let buf = buf_to_slice(buf, len);

    if let Err(e) = diff::command_diff_apply(&dir, buf) {
        eprintln!("Error: {e:#}");
        1
    } else {
        0
    }
}
