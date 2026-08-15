// Application Manager and App System

#[derive(Clone, Copy, Debug)]
pub struct Application {
    pub id: u32,
    pub name: &'static str,
    pub version: &'static str,
    pub enabled: bool,
    pub icon: &'static str,
}

const MAX_APPS: usize = 64;

pub struct AppManager {
    apps: [Option<Application>; MAX_APPS],
    next_id: u32,
    count: usize,
}

impl AppManager {
    pub fn new() -> Self {
        AppManager {
            apps: [None; MAX_APPS],
            next_id: 1,
            count: 0,
        }
    }

    pub fn install_app(&mut self, name: &'static str, version: &'static str, icon: &'static str) -> Option<u32> {
        if self.count >= MAX_APPS {
            return None;
        }
        let app = Application {
            id: self.next_id,
            name,
            version,
            enabled: true,
            icon,
        };
        self.apps[self.count] = Some(app);
        self.count += 1;
        let id = self.next_id;
        self.next_id += 1;
        Some(id)
    }

    pub fn enable_app(&mut self, id: u32) {
        for app_slot in &mut self.apps {
            if let Some(app) = app_slot {
                if app.id == id {
                    app.enabled = true;
                    break;
                }
            }
        }
    }

    pub fn disable_app(&mut self, id: u32) {
        for app_slot in &mut self.apps {
            if let Some(app) = app_slot {
                if app.id == id {
                    app.enabled = false;
                    break;
                }
            }
        }
    }

    pub fn list_apps(&self) -> &[Option<Application>] {
        &self.apps[..self.count]
    }
}

pub fn init() {
    crate::println!("[*] Application manager initialized");
}
