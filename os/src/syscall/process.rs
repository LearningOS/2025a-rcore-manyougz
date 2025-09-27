//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

use crate::task::{current_syscall_count, increase_syscall_count};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    increase_syscall_count();
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    increase_syscall_count();
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    increase_syscall_count();
    match _trace_request {
        0 => {
            trace!("Trace request 0, id: {}, data: {}", _id, _data);
            unsafe { 
                return (_id as *const usize).read_volatile() as isize;
            };
        },
        1 => {
            trace!("Trace request 1, id: {}, data: {}", _id, _data);
            unsafe {
                (_id as *mut usize).write_volatile(_data);
            };
            return 0;
        },
        2 => {
            trace!("Trace request 2, id: {}, data: {}", _id, _data);
            return current_syscall_count() as isize;
        }
        _ => trace!("Unknown trace request: {}, id: {}, data: {}", _trace_request, _id, _data),
    }
    trace!("kernel: sys_trace");
    -1
}
