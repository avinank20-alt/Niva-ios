// File Manager Application

const MAX_SELECTED_FILES: usize = 128;

pub struct FileManager {
    current_path: &'static str,
    selected_files: [Option<&'static str>; MAX_SELECTED_FILES],
    selected_count: usize,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            current_path: "/",
            selected_files: [None; MAX_SELECTED_FILES],
            selected_count: 0,
        }
    }

    pub fn navigate(&mut self, path: &'static str) {
        self.current_path = path;
        crate::println!("[*] Navigating to: {}", path);
    }

    pub fn select_file(&mut self, filename: &'static str) {
        if self.selected_count < MAX_SELECTED_FILES {
            self.selected_files[self.selected_count] = Some(filename);
            self.selected_count += 1;
        }
    }

    pub fn copy_file(&self, source: &'static str, destination: &'static str) {
        crate::println!("[*] Copying {} to {}", source, destination);
    }

    pub fn delete_file(&mut self, filename: &'static str) {
        crate::println!("[*] Deleting {}", filename);
    }
}
