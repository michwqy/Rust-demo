#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use libc::{c_char, c_int, c_uchar, size_t, ssize_t, strncmp};
use std::cmp::min;
use std::ffi::{CStr, CString};

const BUFFER_DEFAULT_SIZE: size_t = 64;

//rust version for c string methods
pub fn strlen(str: &[c_char]) -> size_t {
    str.iter()
        .position(|&c| c == 0)
        .expect("input string isn't null terminated") as size_t
}

pub fn strcmp(str1: &[c_char], str2: &[c_char]) -> c_int {
    let num = min(str1.len(), str2.len());
    unsafe { strncmp(str1.as_ptr(), str2.as_ptr(), num) }
}

// returns index instead of pointer
pub fn strstr(str: &[c_char], substr: &[c_char]) -> Option<size_t> {
    let substr = &substr[..substr.len() - 1]; // slice off null-terminator
    str.windows(substr.len()).position(|w| w == substr).or(None)
}

pub fn strncat(dest: &mut [c_char], src: &[c_char], num: size_t) {
    let dlen = strlen(dest);
    let slen = strlen(src);
    let num = min(num, slen);
    let needed = dlen + num + 1;
    if dest.len() < needed {
        panic!("not enough length of dest");
    }
    let ds = &mut dest[dlen..dlen + num];
    let ss = &src[..num];
    ds.copy_from_slice(ss);
    dest[needed - 1] = 0;
}

pub fn isspace(c: c_int) -> c_int {
    match char::from_u32(c as u32).unwrap().is_ascii_whitespace() {
        true => 1 as c_int,
        false => 0 as c_int,
    }
}

pub struct buffer_t {
    pub len: size_t,
    pub alloc: Vec<c_char>,
    pub data: size_t, // points to first char of string in `alloc`
}

impl buffer_t {
    pub unsafe fn data_ptr(&self) -> *const c_char {
        self.alloc.as_ptr().offset(self.data as isize)
    }

    pub unsafe fn data_mut_ptr(&mut self) -> *mut c_char {
        self.alloc.as_mut_ptr().offset(self.data as isize)
    }

    pub fn data_slice(&self) -> &[c_char] {
        &self.alloc[self.data as usize..]
    }

    pub fn data_mut_slice(&mut self) -> &mut [c_char] {
        &mut self.alloc[self.data as usize..]
    }

    pub fn data_str(&self) -> &[c_char] {
        //cut out the extra 0
        let end = self.data + strlen(self.data_slice());
        &self.alloc[self.data as usize..end as usize]
    }
}

//allocate a new buffer with BUFFER_DEFAULT_SIZE
#[no_mangle]
pub extern "C" fn buffer_new() -> *mut buffer_t {
    return buffer_new_with_size(BUFFER_DEFAULT_SIZE);
}

//allocate a new buffer with n bytes
#[no_mangle]
pub extern "C" fn buffer_new_with_size(n: size_t) -> *mut buffer_t {
    let mut self_0 = Box::new(imp_buffer_new_with_size(n));
    Box::into_raw(self_0)
}

pub fn imp_buffer_new_with_size(n: size_t) -> buffer_t {
    let mut self_0 = buffer_t {
        len: 0,
        alloc: vec![],
        data: 0,
    };
    self_0.len = n;
    self_0.alloc = vec![0 as c_char; n as usize + 1];
    self_0.data = 0;
    self_0
}

//allocate a new buffer with copy of string
#[no_mangle]
pub extern "C" fn buffer_new_with_copy(str: *const c_char) -> *mut buffer_t {
    let str = char_to_vec(str);
    let mut self_0 = Box::new(imp_buffer_new_with_copy(&str));
    Box::into_raw(self_0)
}

pub fn imp_buffer_new_with_copy(str: &[c_char]) -> buffer_t {
    let len: size_t = strlen(str);
    let mut self_0 = imp_buffer_new_with_size(len);
    self_0.alloc.clone_from_slice(str);
    self_0.data = 0;
    self_0
}

//free the buffer
#[no_mangle]
pub extern "C" fn buffer_free(mut self_0: *mut buffer_t) {
    let x = unsafe{Box::from_raw(self_0)};
    //x will be auto free after this function
}

//return buffer size
#[no_mangle]
pub extern "C" fn buffer_size(self_0: &buffer_t) -> size_t {
    return self_0.len;
}

//return string length
#[no_mangle]
pub extern "C" fn buffer_length(self_0: &buffer_t) -> size_t {
    let l = strlen(self_0.data_slice());
    //println!("{}", l);
    return l;
}

fn nearest_multiple_of(a: size_t, b: size_t) -> size_t {
    (b + (a - 1)) & !(a - 1)
}

//convert char* into vec<c_char> rather than vec<c_uchar>
fn char_to_vec(ori: *const c_char) -> Vec<c_char> {
    let c_str = unsafe { CStr::from_ptr(ori) };
    let c_str = c_str.to_bytes_with_nul();
    let vec: Vec<c_char> = c_str.iter().map(|b| *b as c_char).collect::<Vec<_>>();
    return vec;
}

//resize the buffer
pub fn buffer_resize(self_0: &mut buffer_t, mut n: size_t) -> c_int {
    n = nearest_multiple_of(1024, n);
    self_0.len = n;
    self_0.data = 0;
    self_0.alloc.resize_with(n + 1, Default::default);
    // This statement is redundant, just for logic
    self_0.alloc[n] = 0;
    return 0;
}

//append string to buffer
#[no_mangle]
pub extern "C" fn buffer_append(self_0: &mut buffer_t, str: *const c_char) -> c_int {
    let str = char_to_vec(str);
    return imp_buffer_append_n(self_0, &str, strlen(&str));
}

//append string to buffer with specific length
#[no_mangle]
pub extern "C" fn buffer_append_n(self_0: &mut buffer_t, str: *const c_char, len: size_t) -> c_int {
    let str = char_to_vec(str);
    return imp_buffer_append_n(self_0, &str, len);
}

pub fn imp_buffer_append_n(self_0: &mut buffer_t, str: &[c_char], len: size_t) -> c_int {
    let prev: size_t = strlen(self_0.data_slice());
    let needed: size_t = len.wrapping_add(prev);
    // enough space
    if self_0.len > needed {
        strncat(self_0.data_mut_slice(), str, len);
        return 0;
    };
    // resize
    let ret = buffer_resize(self_0, needed);
    if -1 == ret {
        return -1;
    };

    strncat(self_0.data_mut_slice(), str, len);
    return 0;
}

//convert vec<c_char> into vec<c_uchar> to accommodate rust methods
pub fn vec_to_char(data: &[c_char]) -> Vec<c_uchar> {
    let vec: Vec<c_uchar> = data.iter().map(|b| *b as c_uchar).collect::<Vec<_>>();
    return vec;
}

//return string in buffer
#[no_mangle]
pub extern "C" fn buffer_string(self_0: &buffer_t) -> *const c_char {
    //can just return ptr for [c_char]
    //let data = vec_to_char(self_0.data_str());
    //let c_str = CString::new(data).unwrap();
    return self_0.data_str().as_ptr();
}

//prepend string to buffer
#[no_mangle]
pub extern "C" fn buffer_prepend(mut self_0: &mut buffer_t, str: *const c_char) -> c_int {
    let str = char_to_vec(str);
    return imp_buffer_prepend(self_0, &str);
}

pub fn imp_buffer_prepend(mut self_0: &mut buffer_t, str: &[c_char]) -> c_int {
    let len: size_t = strlen(str);
    let prev: size_t = strlen(self_0.data_slice());
    let needed: size_t = len.wrapping_add(prev);
    // enough space
    if !(self_0.len > needed) {
        // resize
        let ret = buffer_resize(&mut self_0, needed);
        if -1 == ret {
            return -1;
        }
    }
    //imply using rust safe methods, which is equivalent to c methods
    self_0.data_mut_slice().copy_within(0..(prev + 1), len);
    self_0.data_mut_slice()[..len].copy_from_slice(&str[..len]);
    /*
    unsafe {
        libc::memmove(self_0.data_ptr().offset(len as isize) as *mut libc::c_void,
                self_0.data_ptr() as *const libc::c_void,
                len + 1);
        libc::memcpy(self_0.data_ptr() as *mut libc::c_void,
                     str.as_ptr() as *const libc::c_void,
               len);
    }
    */
    return 0;
}

//return slice of buffer
#[no_mangle]
pub extern "C" fn buffer_slice(
    mut buf: &buffer_t,
    from: size_t,
    mut to: ssize_t,
) -> *mut buffer_t {
    let len: size_t = strlen(buf.data_slice());
    // bad range
    if (to as size_t) < from {
        //Option cannot be processed in c
        return 0 as *mut buffer_t;
    }
    let self_0 = Box::new(imp_buffer_slice(buf, from, to));
    Box::into_raw(self_0)
}

pub fn imp_buffer_slice(mut buf: &buffer_t, from: size_t, mut to: ssize_t) -> buffer_t {
    let mut len: size_t = strlen(buf.data_slice());
    // relative to end
    if to < 0 {
        to = (len - (!to as usize)) as ssize_t
    }
    // compare end
    if to as size_t > len {
        to = len as ssize_t
    }
    let n: size_t = (to as size_t) - from;
    let mut self_0 = imp_buffer_new_with_size(n);
    let src = &buf.data_slice()[from..from + n];
    let dst = &mut self_0.data_mut_slice()[..n];
    dst.copy_from_slice(src);
    self_0
}

//return is equal or not
#[no_mangle]
pub extern "C" fn buffer_equals(self_0: &buffer_t, other: &buffer_t) -> c_int {
    (strcmp(self_0.data_slice(), other.data_slice()) == 0) as c_int
}

//return the index of substring
#[no_mangle]
pub extern "C" fn buffer_indexof(self_0: &buffer_t, str: *const c_char) -> ssize_t {
    let str = char_to_vec(str);
    return imp_buffer_indexof(self_0, &str);
}

pub fn imp_buffer_indexof(self_0: &buffer_t, str: &[c_char]) -> ssize_t {
    let mut sub = strstr(self_0.data_slice(), str);
    if sub.is_none() {
        return -(1 as c_int) as ssize_t;
    }
    return sub.unwrap().wrapping_sub(self_0.data) as ssize_t;
}

//trim leading whitespace
#[no_mangle]
pub extern "C" fn buffer_trim_left(self_0: &mut buffer_t) {
    loop {
        let c = self_0.data_slice()[0] as c_int;
        if !(c != 0 && isspace(c) != 0) {
            break;
        }
        self_0.data += 1;
    }
}

//trim trailing whitespace
#[no_mangle]
pub extern "C" fn buffer_trim_right(mut self_0: &mut buffer_t) {
    let mut c: c_int = 0;
    let mut i: usize = buffer_length(self_0) as usize - 1;
    loop {
        c = self_0.data_slice()[i] as c_int;
        if !(c != 0 && isspace(c) != 0) {
            break;
        }
        self_0.data_mut_slice()[i] = 0;
        i = i - 1;
    }
}

//trim trailing and leading whitespace
#[no_mangle]
pub extern "C" fn buffer_trim(mut self_0: &mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}

//fill the buffer with c
#[no_mangle]
pub extern "C" fn buffer_fill(mut self_0: &mut buffer_t, c: c_char) {
    self_0.data_mut_slice().fill(c);
}

//fill the buffer with 0
#[no_mangle]
pub extern "C" fn buffer_clear(mut self_0: &mut buffer_t) {
    buffer_fill(self_0, 0);
}

//deallocate excess memory, the number of bytes removed or -1
#[no_mangle]
pub extern "C" fn buffer_compact(self_0: &mut buffer_t) -> ssize_t {
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
