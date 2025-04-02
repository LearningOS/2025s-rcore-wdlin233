//! Types related to task management

use alloc::collections::btree_map::BTreeMap;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

#[derive(Clone)]
pub struct SyscallTrace {
    pub syscall_count: BTreeMap<usize, usize>,
}


impl SyscallTrace {
    pub fn new() -> Self {
        Self {
            syscall_count: BTreeMap::new(),
        }
    }
}

impl Default for SyscallTrace {
    fn default() -> Self {
         Self::new()
    }
}