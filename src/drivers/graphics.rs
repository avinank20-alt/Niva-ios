// Graphics/Display driver

pub struct FrameBuffer {
    address: *mut u32,
    width: u32,
    height: u32,
    pitch: u32,
}

impl FrameBuffer {
    pub fn new(address: *mut u32, width: u32, height: u32) -> Self {
        FrameBuffer {
            address,
            width,
            height,
            pitch: width,
        }
    }

    pub fn clear(&mut self, color: u32) {
        unsafe {
            for i in 0..(self.width * self.height) {
                *self.address.add(i as usize) = color;
            }
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            unsafe {
                let offset = (y * self.pitch) + x;
                *self.address.add(offset as usize) = color;
            }
        }
    }
}

pub struct GraphicsDriver;

impl GraphicsDriver {
    pub fn init() {
        crate::println!("[*] Graphics driver initialized");
    }
}
