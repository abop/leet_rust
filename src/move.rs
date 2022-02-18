struct Buffer {
    buffer: String,
}
struct Render {
    current_buffer: Buffer,
    next_buffer: Buffer,
}
//实现结构体 `Render` 的方法
impl Render {
    //实现 update_buffer() 方法，
    //更新buffer，把 next 更新到 current 中，再更新 next
    fn update_buffer(&mut self, buf: String) {
        // self.current_buffer = self.next_buffer;
        // self.next_buffer = Buffer{ buffer: buf};
        self.next_buffer = std::mem::replace(&mut self.current_buffer, Buffer { buffer: buf });
    }
}

pub fn main() {
    let mut _x = Render {
        current_buffer: Buffer {
            buffer: "buf".to_string(),
        },
        next_buffer: Buffer {
            buffer: "buf2".to_string(),
        },
    };
    _x.update_buffer(String::from("newBuf"));
    println!(
        "Hello, world!{}, {}",
        _x.current_buffer.buffer, _x.next_buffer.buffer
    );
}