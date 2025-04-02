//! Process management syscalls
use crate::{
    task::{change_program_brk, exit_current_and_run_next, get_syscall_times, suspend_current_and_run_next}, 
    timer::get_time_us, 
    util::UserSpacePtr
};

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

/// get time with microsecond reimplemented with virtural memory management
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        UserSpacePtr::from(ts).write(TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        });
    }
    0
}

/// A simple syscall tracer reimplemented with virtual memory management
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let addr = id as *mut u8;
            unsafe {
                return UserSpacePtr::from(addr).read() as isize;
            }
        },
        1 => {
            let addr = id as *mut u8;
            unsafe {
                UserSpacePtr::from(addr).write(data as u8);
            }
            return 0;
        },
        2 => {
            return get_syscall_times(id) as isize;
        },
        _ => return -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
