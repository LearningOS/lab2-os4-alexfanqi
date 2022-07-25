//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token};
use crate::task::get_current_task_info;
use crate::timer::get_time_us;
use crate::mm::{try_translate_small_type, translated_large_type, copy_type_into_bufs};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

impl TaskInfo {
    pub fn empty() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, tz: usize) -> isize {
    let us = get_time_us();
    let tmp = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    if let Some(ts) = try_translate_small_type::<TimeVal>(current_user_token(), ts) {
        *ts = tmp;
    }
    else {
        let buffers = translated_large_type::<TimeVal>(current_user_token(), ts);
        unsafe{ copy_type_into_bufs::<TimeVal>(&tmp, buffers) };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    -1
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let mut ti_tmp = TaskInfo::empty();
    let ret = get_current_task_info(&mut ti_tmp);
    if let Some(ti) = try_translate_small_type::<TaskInfo>(current_user_token(), ti) {
        *ti = ti_tmp;
    }
    else {
        let buffers = translated_large_type::<TaskInfo>(current_user_token(), ti);
        unsafe{ copy_type_into_bufs::<TaskInfo>(&ti_tmp, buffers); };
    }
    ret
}
