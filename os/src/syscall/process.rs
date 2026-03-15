//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next};
use crate::task::{task_mmap, task_munmap, current_user_token};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = crate::timer::get_time_us();
    let token = current_user_token();
    
    // 通过查表获取物理页上对应的字节切片数组（能完美处理跨页问题）
    let mut buffers = crate::mm::translated_byte_buffer(token, ts as *const u8, core::mem::size_of::<TimeVal>());
    if buffers.is_empty() {
        return -1;
    }
    
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    
    // 将 time_val 的数据拷贝到被安全翻译过的连续物理切片中
    let src = &time_val as *const TimeVal as *const u8;
    let mut start = 0;
    for buffer in buffers.iter_mut() {
        let len = buffer.len();
        buffer.copy_from_slice(unsafe { core::slice::from_raw_parts(src.add(start), len) });
        start += len;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => { // 读取用户内存的 1 个字节
            let token = current_user_token();
            let buffers = crate::mm::translated_byte_buffer(token, id as *const u8, 1);
            if buffers.is_empty() { return -1; }
            buffers[0][0] as isize
        }
        1 => { // 写入用户内存的 1 个字节
            let token = current_user_token();
            let mut buffers = crate::mm::translated_byte_buffer(token, id as *const u8, 1);
            if buffers.is_empty() { return -1; }
            buffers[0][0] = data as u8;
            0
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    task_mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    task_munmap(start, len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
