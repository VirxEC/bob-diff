use core::slice;
use std::{ffi::CString, fs, path::Path, ptr};

pub fn main() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/");

    let old = base.join("old");
    let new = base.join("new");

    let mut out_buf = ptr::null_mut();
    let mut out_buf_len = 0;

    {
        let cstr_old = CString::new(old.as_os_str().as_encoded_bytes()).unwrap();
        let cstr_new = CString::new(new.as_os_str().as_encoded_bytes()).unwrap();

        let err = unsafe {
            bob_diff::diff(
                cstr_old.as_ptr(),
                cstr_new.as_ptr(),
                &raw mut out_buf,
                &raw mut out_buf_len,
            )
        };

        assert_eq!(err, 0);
    }

    {
        let applied = base.join("applied");
        if !applied.exists() {
            fs::create_dir(&applied).unwrap();
        }

        let applied_file = applied.join("file.txt");
        if applied_file.exists() {
            fs::remove_file(&applied_file).unwrap();
        }

        fs::copy(old.join("file.txt"), &applied_file).unwrap();

        let cstr_applied = CString::new(applied.as_os_str().as_encoded_bytes()).unwrap();
        let err = unsafe { bob_diff::diff_apply(cstr_applied.as_ptr(), out_buf, out_buf_len) };

        assert_eq!(err, 0);

        let new_contents = fs::read_to_string(new.join("file.txt")).unwrap();
        let applied_contents = fs::read_to_string(&applied_file).unwrap();
        assert_eq!(new_contents, applied_contents);
    }

    drop(unsafe { Box::from_raw(slice::from_raw_parts_mut(out_buf, out_buf_len as usize)) });
}
