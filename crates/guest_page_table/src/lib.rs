//! Guest Page Table 
//! Rely on Page Table Crate。
//! 
//! this crate is used for memory virtualization
//! you can use this crate if and only if the feature `hv` is opened

#![no_std]
#![feature(const_trait_impl)]
#![feature(result_option_inspect)]
#![feature(doc_auto_cfg)]

use page_table::PagingIf;
use memory_addr::PhysAddr;

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv.rs"]
pub mod riscv;
#[cfg(target_arch = "x86_64")]
#[path = "arch/x86.rs"]
pub mod x86;
#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch.rs"]
pub mod aarch;

mod bits64;
mod interface;
mod error;

pub use self::bits64::GuestPageTable64Interface;
pub use self::interface::GuestMemoryInterface;
pub use self::interface::{
    GuestPageNum, GuestPhysAddr, GuestVirtAddr,
    HostPageNum, HostPhysAddr, HostVirtAddr
};
pub use self::error::{GuestPageTableResult, GuestPageTableError};

pub trait GuestPagingIf: PagingIf {
    /// Request to allocate `page_nums` 4K-sized physical frame.
    #[cfg(target_arch = "riscv64")]
    fn alloc_frames(page_nums: usize) -> Option<PhysAddr>;

    /// Request to free `page_nums` 4K-sized physical frame.
    #[cfg(target_arch = "riscv64")]
    fn dealloc_frames(paddr: PhysAddr, page_nums: usize);
}