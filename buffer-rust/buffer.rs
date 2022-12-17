#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use libc::*;
use libc::{size_t, ssize_t};
use std::os::raw::c_int;
use std::cmp::min;

const BUFFER_DEFAULT_SIZE: size_t = 64;

pub struct buffer_t {
    pub len: size_t,
    pub alloc: Vec<libc::c_char>,
    pub data: size_t, 
}

impl buffer_t {
    pub unsafe fn data_ptr(&self) -> *const libc::c_char {
        self.alloc.as_ptr().offset(self.data as isize)
    }

    pub unsafe fn data_mut_ptr(&mut self) -> *mut libc::c_char {
        self.alloc.as_mut_ptr().offset(self.data as isize)
    }

    pub fn data_slice(&self) -> &[libc::c_char] {
        &self.alloc[self.data as usize..]
    }

    pub fn data_mut_slice(&mut self) -> &mut [libc::c_char] {
        &mut self.alloc[self.data as usize..]
    }
}


pub fn strlen(str: &[c_char]) -> size_t {
    str
        .iter()
        .position(|&c| c == 0)
        .expect("input string isn't null terminated") as size_t
}

pub fn strcmp(str1: &[c_char], str2: &[c_char]) -> c_int {
    let num = min(str1.len(), str2.len());
    unsafe {
        libc::strncmp(str1.as_ptr(), str2.as_ptr(), num)
    }
}

pub fn strstr(str: &[c_char], substr: &[c_char]) -> Option<size_t> {
    let substr = &substr[..substr.len()-1];
    str.windows(substr.len()).position(|w| w == substr).or(None)
}

pub fn strncat(dest: &mut [c_char], src: &[c_char], num: size_t) {
    let dlen = strlen(dest);
    let slen = strlen(src);
    let num = min(num, slen);
    let needed = dlen + num + 1;
    if dest.len() < needed {
        panic!("the destination slice not large enough ");
    }
    let ds = &mut dest[dlen..dlen + num];
    let ss = &src[..num];
    ds.copy_from_slice(ss);
    dest[needed - 1] = 0; // null terminator
}

pub fn isspace(c: c_int) -> c_int {
    match char::from_u32(c as u32).unwrap().is_ascii_whitespace() {
        true => 1 as c_int,
        false => 0 as c_int
    }
}

#[macro_export]
macro_rules! c_slice {
    ($str:expr) => {{
        let ptr = ::byte_strings::c_str!($str).as_ptr();
        unsafe { std::slice::from_raw_parts(ptr, $str.len() + 1) }
    }};
}


pub fn buffer_new() -> buffer_t {
    return buffer_new_with_size(BUFFER_DEFAULT_SIZE);
}

pub fn buffer_new_with_size(mut n: size_t) -> buffer_t {
    let mut self_0 = buffer_t { len: 0, alloc: vec![], data: 0 };
    self_0.len = n;
    self_0.alloc = vec![0 as libc::c_char; n as usize + 1];
    self_0.data = 0;
    self_0
}

pub fn buffer_new_with_string(str: Vec<libc::c_char>) -> buffer_t {
    let len = strlen(str.as_ref());
    return buffer_new_with_string_length(str, len as size_t);
}

pub fn buffer_new_with_string_length(str: Vec<libc::c_char>, len: size_t) -> buffer_t {
    let mut self_0 = buffer_t { len: 0, alloc: vec!{}, data: 0 };
    self_0.len = len;
    self_0.alloc = str;
    self_0.data = 0;
    self_0
}

#[no_mangle]
pub fn buffer_new_with_copy(mut str: &[libc::c_char]) -> buffer_t {
    let mut len: size_t = strlen(str);
    let mut self_0: buffer_t = buffer_new_with_size(len);
    self_0.alloc.clone_from_slice(str);
    self_0.data = 0;
    return self_0;
}

pub fn buffer_compact(self_0: &mut buffer_t) -> ssize_t {
    let len: size_t = buffer_length(self_0);
    let rem: size_t = self_0.len.wrapping_sub(len);
    let mut buf = vec![0; len as usize + 1];
    let t = self_0.data_slice();
    buf.clone_from_slice(&t[..len as usize + 1]);
    self_0.len = len;
    self_0.alloc = buf;
    self_0.data = 0;
    return rem as ssize_t;
}

pub fn buffer_free(mut _self_0: buffer_t) {
}

pub fn buffer_size(self_0: &buffer_t) -> size_t {
    return self_0.len;
}

pub fn buffer_length(self_0: &buffer_t) -> size_t {
    strlen(self_0.data_slice())
}


fn nearest_multiple_of(a: size_t, b: size_t) -> size_t {
    (b + (a - 1)) & !(a - 1)
}

pub fn buffer_resize(
    self_0: &mut buffer_t,
    mut n: size_t) -> libc::c_int {
    n = nearest_multiple_of(1024, n);
    self_0.len = n;
    self_0.data = 0;
    self_0.alloc.resize_with(n + 1, Default::default);
    self_0.alloc[n] = 0;
    return 0;
}


pub fn buffer_append(mut self_0: &mut buffer_t, str: &[libc::c_char]) -> libc::c_int {
    return buffer_append_n(self_0, str, strlen(str));
}

pub fn buffer_append_n(
    self_0: &mut buffer_t,
    str: &[libc::c_char],
    len: size_t,
) -> libc::c_int {
    let mut prev: size_t = strlen(self_0.data_slice());
    let mut needed: size_t = len.wrapping_add(prev);
    if self_0.len > needed {
        strncat(self_0.data_mut_slice(), str, len);
        return 0;
    };
    let ret = buffer_resize(self_0, needed);
    if -1 == ret {
        return -1;
    };

    strncat(self_0.data_mut_slice(), str, len);
    return 0;
}

pub fn buffer_prepend(
    mut self_0: &mut buffer_t,
    mut str: &[libc::c_char],
) -> libc::c_int {
    let mut len: size_t = strlen(str);
    let mut prev: size_t = strlen(self_0.data_slice());
    let mut needed: size_t = len.wrapping_add(prev);
    if !(self_0.len > needed) {
        let ret = buffer_resize(&mut self_0, needed);
        if -1 == ret { return -1 }
    }
    unsafe {
        libc::memmove(self_0.data_ptr().offset(len as isize) as *mut libc::c_void,
                self_0.data_ptr() as *const libc::c_void,
                len + 1);
        libc::memcpy(self_0.data_ptr() as *mut libc::c_void,
                     str.as_ptr() as *const libc::c_void,
               len);
    }
    return 0 as libc::c_int;
}

pub fn buffer_slice(
    mut buf: &buffer_t,
    mut from: size_t,
    mut to: ssize_t,
) -> Option<buffer_t> {
    let mut len: size_t = strlen(buf.data_slice());
    if (to as size_t) < from { return None }
    if to < 0 {
        to = (len - (!to as usize)) as ssize_t
    }
    if to as size_t > len { to = len as ssize_t }
    let mut n: size_t = (to as size_t)- from;
    let mut self_0 = buffer_new_with_size(n);
    let src = &buf.data_slice()[from..from + n];
    let dst = &mut self_0.data_mut_slice()[..n];
    dst.copy_from_slice(src);
    return Some(self_0);
}

pub fn buffer_equals(mut self_0: &buffer_t, mut other: &buffer_t) -> libc::c_int {
    (strcmp(self_0.data_slice(), other.data_slice()) == 0) as c_int
}

pub fn buffer_indexof(
    self_0: &buffer_t,
    str: &[libc::c_char],
) -> ssize_t {
    let mut sub = strstr(self_0.data_slice(), str);
    if sub.is_none() { return -(1 as libc::c_int) as ssize_t }
    return sub.unwrap().wrapping_sub(self_0.data) as ssize_t;
}

pub fn buffer_trim_left(self_0: &mut buffer_t) {
    loop {
        let mut c = self_0.data_slice()[0] as libc::c_int;
        if !(c != 0 && isspace(c) != 0) {
            break;
        }
        self_0.data += 1;
    }
}

pub fn buffer_trim_right(mut self_0: &mut buffer_t) {
    let mut c: libc::c_int = 0;
    let mut i: usize = buffer_length(self_0) as usize - 1;
    loop {
        c = self_0.data_slice()[i] as libc::c_int;
        if !(c != 0 && isspace(c) != 0) {
            break;
        }
        self_0.data_mut_slice()[i] = 0;
        i = i - 1;
    }
}

pub fn buffer_trim(self_0: &mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}

pub fn buffer_fill(mut self_0: &mut buffer_t, mut c: libc::c_int) {
    self_0.data_mut_slice().fill(c.try_into().unwrap());

}

#[no_mangle]
pub fn buffer_clear(mut self_0: &mut buffer_t) {
    buffer_fill(self_0, 0);
}


