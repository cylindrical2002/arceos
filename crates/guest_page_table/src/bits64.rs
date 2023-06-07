extern crate alloc;

use super::GuestPagingIf;
use alloc::vec;
use memory_addr::{PhysAddr, PAGE_SIZE_4K};
use page_table::{GenericPTE, PageTable64, PagingError, PagingMetaData, PagingResult};

pub trait GuestPageTable64Interface<M: PagingMetaData, PTE: GenericPTE, IF: GuestPagingIf> : Sized {
    #[cfg(target_arch = "riscv64")]
    fn try_new_gpt() -> PagingResult<Self>;
}

impl<M: PagingMetaData, PTE: GenericPTE, IF: GuestPagingIf> GuestPageTable64Interface<M, PTE, IF>
    for PageTable64<M, PTE, IF>
{
    /// GuestPageTable 16KiB support
    ///
    /// Only Used for Allocate a new guest OS paging
    #[cfg(target_arch = "riscv64")]
    fn try_new_gpt() -> PagingResult<Self> {
        let root_paddr = Self::alloc_guest_page_table()?;
        Ok(Self::new(
            root_paddr,
            vec![
                root_paddr,
                root_paddr + 0x1000,
                root_paddr + 0x2000,
                root_paddr + 0x3000,
            ],
        ))
    }
}

trait GuestPageTable64Private<M: PagingMetaData, PTE: GenericPTE, IF: GuestPagingIf> {
    #[cfg(target_arch = "riscv64")]
    fn alloc_guest_page_table() -> PagingResult<PhysAddr>;
}

// Private implements.
impl<M: PagingMetaData, PTE: GenericPTE, IF: GuestPagingIf> GuestPageTable64Private<M, PTE, IF>
    for PageTable64<M, PTE, IF>
{
    #[cfg(target_arch = "riscv64")]
    fn alloc_guest_page_table() -> PagingResult<PhysAddr> {
        if let Some(paddr) = IF::alloc_frames(4) {
            let ptr = IF::phys_to_virt(paddr).as_mut_ptr();
            unsafe { core::ptr::write_bytes(ptr, 0, PAGE_SIZE_4K * 4) };
            Ok(paddr)
        } else {
            Err(PagingError::NoMemory)
        }
    }
}
