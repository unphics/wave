#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_float, c_int};

// 定义C函数接口
#[link(name = "RecastNav")]
extern "C" {
    /**
     * 1.初始化Recast引擎——recast_init
     * 2.加载地图——recast_loadmap(int id, const char* path)，id为地图的id，因为我们某些游戏中可能会有多个地图寻路实
     *   例，例如Moba游戏，每一场游戏中的地图寻路都是独立的，需要id来区分，path就是寻路数据的完整路径（包含文件名），
     *   这个寻路数据我们可以通过RecastDemo来得到
     * 3.寻路——recast_findpath(int id, const float* spos, const float* epos)，寻路的结果其实只是返回从起点到终点之
     *   间所有经过的凸多边形的序号，id为地图id，spos为起始点，epos为中点，我们可以把它们理解为C#中的Vector3
     * 4.计算实际路径——recast_smooth(int id, float step_size, float slop)，计算平滑路径，其实是根据findpath得到的
     *   【从起点到终点所经过的凸多边形的序号】，得到真正的路径（三维坐标），所以这一步是不可缺少的
     * 5.得到凸多边形id序列——recast_getpathpoly(int id)，得到pathfind以后，从起点到终点所经过的所有凸多边形id的序列
     * 6.得到寻路路径坐标序列——recast_getpathsmooth(int id)，得到smooth以后，路线的三维坐标序列
     * 7.释放地图——recast_freemap(int id)，游戏结束后记得释放地图寻路数据资源嗷
     * 8.释放Recast引擎——recast_fini()，如果我们在客户端使用，游戏流程结束要使用这个释放Recast引擎
     */
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
