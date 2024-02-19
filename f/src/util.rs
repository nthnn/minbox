extern crate libc;

use libc::{getpwuid_r, passwd, c_char};

use std::ptr;
use std::str;

pub fn sys_username(uid: u32) -> Option<String> {
    unsafe {
        let mut passwd_buf: [u8; 1024] = [0; 1024];
        let mut passwd_result: passwd = std::mem::zeroed();
        let mut passwd_string: Option<String> = None;

        let mut result: *mut passwd = ptr::null_mut::<passwd>();
        if getpwuid_r(
            uid,
            &mut passwd_result,
            passwd_buf.as_mut_ptr() as *mut c_char, 
            passwd_buf.len(),
            &mut result
        ) == 0 && !result.is_null() {
            let username_bytes: &[u8] = std::ffi::CStr::from_ptr((*result).pw_name).to_bytes();

            if let Ok(username) = str::from_utf8(username_bytes) {
                passwd_string = Some(username.to_string());
            }
        }

        passwd_string
    }
}

pub fn mode_str(mode: u32) -> String {
    let mut result: String = String::with_capacity(11);
    let permissions: [char; 3] = ['r', 'w', 'x'];

    result.push('-');
    for i in 0..3 {
        for j in 0..3 {
            let bit: u32 = mode & (1 << (8 - (i * 3 + j)));

            if bit == 0 {
                result.push('-');
            }
            else {
                result.push(permissions[j as usize]);
            }
        }
    }

    result
}

pub fn pad_str(mut s: String, len: usize) -> String {
    while s.len() < len {
        s.push(' ');
    }

    s
}