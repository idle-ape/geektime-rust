use std::alloc::{GlobalAlloc, System};
use std::io::Write;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let data = unsafe { System.alloc(layout) };
        let mut buf = [0u8; 128];
        let mut cursor = &mut buf[..];
        let _ = write!(cursor, "ALLOC: {:p}, size: {}\n", data, layout.size());
        let written = 128 - cursor.len();
        unsafe { libc::write(2, buf.as_ptr() as *const _, written) };
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        unsafe { System.dealloc(ptr, layout) };
        let mut buf = [0u8; 128];
        let mut cursor = &mut buf[..];
        let _ = write!(cursor, "FREE: {:p}, size: {}\n", ptr, layout.size());
        let written = 128 - cursor.len();
        unsafe { libc::write(2, buf.as_ptr() as *const _, written) };
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

fn main() {
    let data = Box::new(Matrix::default());

    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );

    // data在这里 drop，可以在打印中看到 FREE
    // 之后还有很多其他内存被释放
}
