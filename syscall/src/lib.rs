#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![feature(asm)]

pub mod arch;

pub const ADDR_SPACE_CREATE: usize = 0x100;
pub const ADDR_SPACE_DESTROY: usize = 0x101;
pub const ADDR_SPACE_CREATE_MAPPING: usize = 0x102;
pub const ADDR_SPACE_DESTROY_MAPPING: usize = 0x103;

pub const TASK_CREATE: usize = 0x200;
pub const TASK_DESTROY: usize = 0x201;

pub const MUTEX_CREATE: usize = 0x300;
pub const MUTEX_DESTROY: usize = 0x301;

pub const IPC_PORT_CREATE: usize = 0x400;
pub const IPC_PORT_DESTROY: usize = 0x401;
pub const IPC_PORT_CALL_SYNC: usize = 0x402;
pub const IPC_PORT_RETURN_SYNC: usize = 0x403;

pub const DEBUG_PRINT_CHAR: usize = 0x500;
