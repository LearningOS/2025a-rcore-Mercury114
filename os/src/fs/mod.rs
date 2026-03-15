//! File trait & inode(dir, file, pipe, stdin, stdout)

mod inode;
mod stdio;
mod file_status;  //新增这一行

use crate::mm::UserBuffer;

/// trait File for all file types
pub trait File: Send + Sync {
    /// the file readable?
    fn readable(&self) -> bool;
    /// the file writable?
    fn writable(&self) -> bool;
    /// read from the file to buf, return the number of bytes read
    fn read(&self, buf: UserBuffer) -> usize;
    /// write to the file from buf, return the number of bytes written
    fn write(&self, buf: UserBuffer) -> usize;
    /// return file status
    fn status(&self) -> FileStatus {
        unimplemented!() // for Stdin and Stdout
    }
}

/*
/// The stat of a inode
#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    /// ID of device containing file
    pub dev: u64,
    /// inode number
    pub ino: u64,
    /// file type and mode
    pub mode: StatMode,
    /// number of hard links
    pub nlink: u32,
    /// unused pad
    pad: [u64; 7],
}
    */

pub use file_status::{FileStatus, Stat, StatMode};
pub use inode::{list_apps, open_file, link_at, unlink_at, OSInode, OpenFlags};
pub use stdio::{Stdin, Stdout};
