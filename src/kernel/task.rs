// Task/Process management

#[derive(Clone, Copy, Debug)]
pub struct TaskId(u64);

pub struct Task {
    id: TaskId,
    name: &'static str,
    state: TaskState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

impl Task {
    pub fn new(id: u64, name: &'static str) -> Self {
        Task {
            id: TaskId(id),
            name,
            state: TaskState::Ready,
        }
    }

    pub fn state(&self) -> TaskState {
        self.state
    }
}

pub struct TaskManager {
    next_id: u64,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { next_id: 1 }
    }

    pub fn create_task(&mut self, _name: &'static str) -> TaskId {
        let id = TaskId(self.next_id);
        self.next_id += 1;
        id
    }
}
