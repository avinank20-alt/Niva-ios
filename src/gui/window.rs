// Window management system

#[derive(Clone, Copy, Debug)]
pub struct WindowId(u32);

#[derive(Clone, Copy, Debug)]
pub struct Window {
    pub id: WindowId,
    pub title: &'static str,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

impl Window {
    pub fn new(id: u32, title: &'static str, x: i32, y: i32, width: u32, height: u32) -> Self {
        Window {
            id: WindowId(id),
            title,
            x,
            y,
            width,
            height,
            visible: true,
        }
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn minimize(&mut self) {
        self.visible = false;
    }

    pub fn restore(&mut self) {
        self.visible = true;
    }
}

const MAX_WINDOWS: usize = 16;

pub struct WindowManager {
    windows: [Option<Window>; MAX_WINDOWS],
    next_id: u32,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            windows: [None; MAX_WINDOWS],
            next_id: 1,
        }
    }

    pub fn create_window(&mut self, title: &'static str, x: i32, y: i32, width: u32, height: u32) -> Option<WindowId> {
        for window_slot in &mut self.windows {
            if window_slot.is_none() {
                let id = self.next_id;
                self.next_id += 1;
                let window = Window::new(id, title, x, y, width, height);
                *window_slot = Some(window);
                return Some(WindowId(id));
            }
        }
        None
    }

    pub fn get_window(&self, id: WindowId) -> Option<&Window> {
        self.windows.iter().find_map(|w| {
            w.as_ref().filter(|window| window.id.0 == id.0)
        })
    }

    pub fn close_window(&mut self, id: WindowId) {
        for window_slot in &mut self.windows {
            if let Some(window) = window_slot {
                if window.id.0 == id.0 {
                    window.close();
                    return;
                }
            }
        }
    }
}
