//! Process management syscalls
use crate::{
    config::PAGE_SIZE, mm::{copy_to_virt, trace_read, trace_write}, task::{change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_times, mmap, munmap, suspend_current_and_run_next}, timer::get_time_us,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    let time_val = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
    };

    copy_to_virt(&time_val, ts);
    0
}

/// A simple syscall tracer reimplemented with virtual memory management
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            trace!("sys_trace read with id {}", id);
            let addr = id as *const u8;
            if let Some(value) = trace_read(current_user_token(), addr as usize) {
                trace!("sys_trace read value is {}", value);
                value as isize
            } else {
                -1
            }
        },
        1 => {
            trace!("sys_trace write");
            let addr = id as *const u8;
            if trace_write(current_user_token(), addr as usize, data) {
                0
            } else {
                -1
            }
        },
        2 => {
            trace!("sys_trace syscall");
            return get_syscall_times(id) as isize;
        },
        _ => return -1,
    }
}

/// mmap syscall
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    const PORT_MASK: usize = 0b111;
    
    let aligned_start = start % PAGE_SIZE == 0;
    let port_valid = (port & !PORT_MASK) == 0;
    let port_not_none = (port & PORT_MASK) != 0;
    
    trace!("each condition: aligned_start={}, port_valid={}, port_not_none={}", aligned_start, port_valid, port_not_none);
    if aligned_start && port_valid && port_not_none {
        return mmap(start, len, port)
    }
    -1
}

/// munmap syscall
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let aligned_start = start % PAGE_SIZE == 0;
    if aligned_start {
        return munmap(start, len)
    }
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
