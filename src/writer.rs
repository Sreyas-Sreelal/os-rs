pub struct Writer {
    vga_buf: *mut u8,
    row_pos: usize,
    col_pos: usize,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            vga_buf: 0xb8000 as *mut u8,
            row_pos: 0,
            col_pos: 0,
        }
    }
    pub fn write(&mut self, data: &str) {
        for (i, &byte) in data.as_bytes().iter().enumerate() {
            unsafe {
                *(self.vga_buf).offset(i as isize * 2) = byte;
                *(self.vga_buf).offset(i as isize * 2 + 1) = 0x4;
            }
        }
    }
}
