#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused
)]

use libc::{size_t, ssize_t};
use buffer::*;
use byte_strings::c_str;

extern "C" {
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}

#[no_mangle]
fn equal(a: *const libc::c_char, b: *const libc::c_char) {
    unsafe {
        if strcmp(a, b) != 0 {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(
                b"  expected: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                a,
            );
            printf(
                b"    actual: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                b,
            );
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        };
    }
}

fn test_buffer_new() {
    let mut buf: buffer_t = buffer_new();
    assert!(64 == buffer_size(&buf));
    assert!(0 == buffer_length(&buf));
    buffer_free(buf);
}

fn test_buffer_new_with_size() {
    let mut buf = buffer_new_with_size(1024 as libc::c_int as size_t);
    assert!(1024 == buffer_size(&buf));
    assert!(0 == buffer_length(&buf));
    buffer_free(buf);
}

fn test_buffer_append() {
    let mut buf: buffer_t = buffer_new();
    assert!(0 == buffer_append(&mut buf, c_slice!(b"Hello")));
    assert!(0 == buffer_append(&mut buf, c_slice!(b" World")));
    assert!(
        buffer::strlen(c_slice!(b"Hello World"))
            == buffer_length(&buf)
    );
    equal(
        b"Hello World\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        unsafe { buf.data_ptr() },
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() {
    let mut buf = buffer_new();
    assert!(0 == buffer_append_n(&mut buf, c_slice!(b"subway"), 3));
    assert!(0 == buffer_append_n(&mut buf, c_slice!(b"marines"), 6));
    assert!(buffer::strlen(c_slice!("submarine")) == buffer_length(&buf));
    equal(c_str!("submarine").as_ptr(), buf.data_ptr());
    buffer_free(buf);
}

pub unsafe fn test_buffer_prepend() {
    let mut buf = buffer_new();
    assert_eq!(0, buffer_append(&mut buf,
                                c_slice!(b" World")));
    assert_eq!(0, buffer_prepend(&mut buf,
                                c_slice!(b"Hello")));
    assert_eq!(
        strlen(c_slice!(b"Hello World").as_ptr()) as usize,
        buffer_length(&buf)
    );

    equal(c_slice!(b"Hello World").as_ptr(), buf.data_ptr());
    buffer_free(buf);
}
pub fn test_buffer_slice() {
    let mut buf = buffer_new();
    buffer_append(&mut buf,
                  c_slice!(b"Tobi Ferret"));

    let mut a = buffer_slice(&buf, 2,8).unwrap();
    unsafe {
        equal(c_slice!(b"Tobi Ferret").as_ptr(), buf.data_ptr());
        equal(c_slice!(b"bi Fer").as_ptr(), a.data_ptr());
    }
    buffer_free(buf);
    buffer_free(a);
}

#[no_mangle]
pub unsafe extern "C" fn test_buffer_equals() {
    let mut a = buffer_new_with_copy(c_slice!(b"Hello"));
    let mut b = buffer_new_with_copy(c_slice!(b"Hello"));
    assert!(1 == buffer_equals(&a, &b));

    buffer_append(&mut b, c_slice!(b" World"));
    assert!(0 == buffer_equals(&a, &b));
    buffer_free(a);
    buffer_free(b);
}

#[no_mangle]
pub unsafe extern "C" fn test_buffer_indexof() {
    let mut buf = buffer_new_with_copy(c_slice!(b"Tobi is a ferret"));
    let mut i = buffer_indexof(&buf, c_slice!(b"is"));
    assert_eq!(5, i);
    i = buffer_indexof(&buf, c_slice!(b"a"));
    assert_eq!(8, i);
    i = buffer_indexof(&buf, c_slice!(b"something"));
    assert_eq!(-1, i);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_fill() {
    let mut buf = buffer_new_with_copy(c_slice!(b"Hello"));
    assert_eq!(5, buffer_length(&buf));
    buffer_fill(&mut buf, 0 as libc::c_int);
    assert_eq!(0, buffer_length(&buf));
    buffer_free(buf);
}
#[no_mangle]
pub fn test_buffer_clear() {
    let mut buf = buffer_new_with_copy(c_slice!(b"Hello"));
    assert_eq!(5, buffer_length(&buf));
    buffer_clear(&mut buf);
    assert_eq!(0, buffer_length(&buf));
    buffer_free(buf);
}
#[no_mangle]
pub unsafe fn test_buffer_trim() {
    let mut buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n"));
    buffer_trim(&mut buf);
    equal(
        c_slice!(b"Hello").as_ptr(),
        buf.data_ptr(),
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n "));
    buffer_trim_left(&mut buf);
    equal(
        c_slice!(b"Hello\n\n ").as_ptr(),
        buf.data_ptr(),
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n "),
    );
    buffer_trim_right(&mut buf);
    equal(
        c_slice!(b"  Hello").as_ptr(),
        buf.data_ptr(),
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe fn test_buffer_compact() {
    let mut buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n "));
    buffer_trim(&mut buf);
    assert_eq!(5, buffer_length(&buf));
    assert_eq!(10, buffer_size(&buf));
    let mut removed: ssize_t = buffer_compact(&mut buf);
    assert_eq!(5, removed);
    assert_eq!(5, buffer_length(&buf));
    assert_eq!(5, buffer_size(&buf));
    equal(c_slice!(b"Hello").as_ptr(), buf.data_ptr());
    buffer_free(buf);
}
unsafe fn main_0() -> libc::c_int {
    test_buffer_new();
    test_buffer_new_with_size();
    test_buffer_append();
    test_buffer_append_n();
    test_buffer_slice();
    test_buffer_equals();
    test_buffer_indexof();
    test_buffer_fill();
    test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    printf(c_str!(b"\n  \x1b[32m\xe2\x9c\x93 \x1b[90mok\x1b[0m\n\n").as_ptr());
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
