use std::fmt;

#[allow(unused)]
#[derive(Clone, Copy)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(value: Vec<u8>) -> Self {
        let slice = value.into_boxed_slice();

        Self {
            len: slice.len(),
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_raw_buffer() {
        let data = vec![1, 2, 3, 4, 5];
        let buf: RawBuffer = data.into();
        // 因为 buf 允许 Copy，所以这里 Copy 了一份
        use_buffer(buf);

        // buf 还能用
        println!("buf: {:?}", buf);
    }
    #[allow(dropping_copy_types)]
    fn use_buffer(buf: RawBuffer) {
        println!("buf to die: {:?}", buf);

        // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
        drop(buf)
    }
}
