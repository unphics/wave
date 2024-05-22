#![allow(non_camel_case_types)]

extern crate libc;

use std::os::raw::{c_char, c_float, c_int};

// 定义C函数接口
#[link(name = "RecastNavLib")]
extern "C" {

    pub fn recast_init() -> bool;
    pub fn recast_fini();
    pub fn recast_loadmap(id: c_int, path: *const c_char) -> bool;
    pub fn recast_freemap(id: c_int) -> bool;
    pub fn recast_findpath(id: c_int, spos: *const c_float, epos: *const c_float) -> c_int;
    pub fn recast_smooth(id: c_int, step_size: c_float, slop: c_float) -> bool;
    pub fn recast_raycast(id: c_int, spos: *const c_float, epos: *const c_float) -> c_int;
    pub fn recast_gethitposition(id: c_int) -> *mut c_float;
    pub fn recast_getcountpoly(id: c_int) -> c_int;
    pub fn recast_getcountsmooth(id: c_int) -> c_int;
    pub fn recast_getpathpoly(id: c_int) -> *mut u32;
    pub fn recast_getpathsmooth(id: c_int) -> *mut c_float;
    pub fn recast_getfixposition(id: c_int, pos: *const c_float) -> *mut c_float;
}

// Rust封装函数（可选）
// pub fn load_map(id: i32, path: &str) -> bool {
//     let c_path = std::ffi::CString::new(path).expect("CString::new failed");
//     unsafe { recast_loadmap(id, c_path.as_ptr()) }
// }

// 更多的封装函数可以按照需求定义
