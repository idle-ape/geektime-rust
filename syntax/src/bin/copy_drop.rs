use std::{fmt::Debug, slice};

// 通过 derive 实现 Copy，因为 *mut u8 和 usize 都支持 Copy
#[derive(Copy, Clone)]
struct RawBuffer {
    ptr: *mut u8, // 裸指针用 *mut / *const 来表述，这和引用的 & 不同
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 不能再为 RawBuffer 实现 Drop 了，因为已经为它实现了 Copy，Copy 和 Drop 不能同时实现
// impl Drop for RawBuffer {
//     fn drop(&mut self) {
//         let data = unsafe {
//             Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len))
//         };
//         drop(data);
//     }
// }

impl Debug for RawBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let buf: RawBuffer = data.into();

    use_buffer(buf);
    // buf 还能用，因为 RawBuffer 实现了 Copy 语义，传参通过 copy 而不是 move
    println!("buf: {:?}", buf);
}
