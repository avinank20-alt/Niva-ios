// Taskbar component

#[derive(Clone, Copy, Debug)]
pub struct TaskbarItem {
    pub name: &'static str,
    pub icon: &'static str,
    pub pinned: bool,
}

const MAX_TASKBAR_ITEMS: usize = 32;

pub struct Taskbar {
    pub height: u32,
    items: [Option<TaskbarItem>; MAX_TASKBAR_ITEMS],
    count: usize,
}

impl Taskbar {
    pub fn new(height: u32) -> Self {
        Taskbar {
            height,
            items: [None; MAX_TASKBAR_ITEMS],
            count: 0,
        }
    }

    pub fn add_item(&mut self, name: &'static str, icon: &'static str) {
        if self.count < MAX_TASKBAR_ITEMS {
            self.items[self.count] = Some(TaskbarItem {
                name,
                icon,
                pinned: false,
            });
            self.count += 1;
        }
    }

    pub fn pin_item(&mut self, index: usize) {
        if index < self.count {
            if let Some(item) = &mut self.items[index] {
                item.pinned = true;
            }
        }
    }

    pub fn get_items(&self) -> &[Option<TaskbarItem>] {
        &self.items[..self.count]
    }
}

pub fn init() {
    crate::println!("[*] Taskbar initialized");
}
