// Virtual File System

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
}

#[derive(Clone, Debug)]
pub struct Inode {
    pub number: u32,
    pub file_type: FileType,
    pub size: u32,
    pub permissions: u16,
}

#[derive(Clone, Debug)]
pub struct DirEntry {
    pub name: &'static str,
    pub inode: u32,
}

pub struct VirtualFileSystem {
    pub root_inode: Inode,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        VirtualFileSystem {
            root_inode: Inode {
                number: 0,
                file_type: FileType::Directory,
                size: 4096,
                permissions: 0o755,
            },
        }
    }

    pub fn read_inode(&self, number: u32) -> Option<Inode> {
        if number == 0 {
            Some(self.root_inode.clone())
        } else {
            None
        }
    }

    pub fn create_file(&mut self, _path: &str, file_type: FileType) -> Result<Inode, &'static str> {
        Ok(Inode {
            number: 1,
            file_type,
            size: 0,
            permissions: 0o644,
        })
    }
}

pub fn init() {
    crate::println!("[*] Virtual filesystem initialized");
}
