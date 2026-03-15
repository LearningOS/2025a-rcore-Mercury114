//! util.rs - A Utility Rust Library For rcore
#![allow(missing_docs)]

use core::{mem::size_of, ptr::addr_of};
use alloc::vec::Vec;
use crate::{
    mm::translated_byte_buffer,
    task::current_user_token
};

/// User space ptr wrapper
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserSpacePtr<T>(*mut T);

impl<T: Sized> UserSpacePtr<T> {
    pub unsafe fn write(self, val: T)
    where
        T: Sized,
    {
        let buffers = self.into_buffers();
        let mut src = unsafe {core::slice::from_raw_parts(addr_of!(val) as _, size_of::<T>() )};

        assert_eq!(src.len(), buffers.iter().map(|v| v.len()).sum());
        for buffer in buffers {
            let nbytes = buffer.len();
            buffer.copy_from_slice(&src[..nbytes]);
            src = &src[nbytes..];
        }
        assert_eq!(src.len(), 0);
    }

    pub unsafe fn read(self) -> T
    where
        T: Sized,
    {
        todo!()
    }

    fn into_buffers(self) -> Vec<&'static mut [u8]> {
        translated_byte_buffer(current_user_token(), self.0 as _, size_of::<T>())
    }
}

impl<T> From<*mut T> for UserSpacePtr<T> {
    fn from(value: *mut T) -> Self {
        Self(value)
    }
}