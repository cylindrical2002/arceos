//! Page table manipulation.

use axalloc::global_allocator;
use guest_page_table::{
    GuestMemoryInterface, GuestPageTable64Interface, GuestPageTableError, GuestPageTableResult,
    GuestPagingIf, GuestPhysAddr, HostPhysAddr,
};
use page_table::PagingIf;

use crate::mem::{phys_to_virt, virt_to_phys, MemRegionFlags, PhysAddr, VirtAddr, PAGE_SIZE_4K};

#[doc(no_inline)]
pub use page_table::{MappingFlags, PageSize, PagingError, PagingResult};

impl From<MemRegionFlags> for MappingFlags {
    fn from(f: MemRegionFlags) -> Self {
        let mut ret = Self::empty();
        if f.contains(MemRegionFlags::READ) {
            ret |= Self::READ;
        }
        if f.contains(MemRegionFlags::WRITE) {
            ret |= Self::WRITE;
        }
        if f.contains(MemRegionFlags::EXECUTE) {
            ret |= Self::EXECUTE;
        }
        if f.contains(MemRegionFlags::DEVICE) {
            ret |= Self::DEVICE;
        }
        ret
    }
}

/// Implementation of [`PagingIf`], to provide physical memory manipulation to
/// the [page_table] crate.
pub struct PagingIfImpl;

impl PagingIf for PagingIfImpl {
    fn alloc_frame() -> Option<PhysAddr> {
        global_allocator()
            .alloc_pages(1, PAGE_SIZE_4K)
            .map(|vaddr| virt_to_phys(vaddr.into()))
            .ok()
    }

    // #[cfg(target_arch = "riscv64")]
    // fn alloc_frames(page_nums: usize) -> Option<PhysAddr> {
    //     global_allocator()
    //         .alloc_pages(page_nums, PAGE_SIZE_4K * page_nums)
    //         .map(|vaddr| virt_to_phys(vaddr.into()))
    //         .ok()
    // }

    fn dealloc_frame(paddr: PhysAddr) {
        global_allocator().dealloc_pages(phys_to_virt(paddr).as_usize(), 1)
    }

    // #[cfg(target_arch = "riscv64")]
    // fn dealloc_frames(paddr: PhysAddr, page_nums: usize) {
    //     global_allocator().dealloc_pages(phys_to_virt(paddr).as_usize(), page_nums)
    // }

    #[inline]
    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
        phys_to_virt(paddr)
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        /// The architecture-specific page table.
        pub type PageTable = page_table::x86_64::X64PageTable<PagingIfImpl>;
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        /// The architecture-specific page table.
        pub type PageTable = page_table::riscv::Sv39PageTable<PagingIfImpl>;
    } else if #[cfg(target_arch = "aarch64")]{
        /// The architecture-specific page table.
        pub type PageTable = page_table::aarch64::A64PageTable<PagingIfImpl>;
    }
}

///
/// Hypervisor Code, for Guest PageTable
///
#[cfg(feature = "hv")]
impl GuestPagingIf for PagingIfImpl {
    #[cfg(target_arch = "riscv64")]
    fn alloc_frames(page_nums: usize) -> Option<PhysAddr> {
        global_allocator()
            .alloc_pages(page_nums, PAGE_SIZE_4K * page_nums)
            .map(|vaddr| virt_to_phys(vaddr.into()))
            .ok()
    }

    #[cfg(target_arch = "riscv64")]
    fn dealloc_frames(paddr: PhysAddr, page_nums: usize) {
        global_allocator().dealloc_pages(phys_to_virt(paddr).as_usize(), page_nums)
    }
}

#[cfg(feature = "hv")]
pub type GuestPagingIfImpl = PagingIfImpl;

#[cfg(feature = "hv")]
cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        /// The architecture-specific page table.
        pub type GuestPageTableStub = guest_page_table::x86_64::X64PageTable<PagingIfImpl>;
    } else if #[cfg(target_arch = "riscv64")] {
        /// The architecture-specific guest page table.
        pub type GuestPageTableStub = guest_page_table::riscv::Sv39x4PageTable<GuestPagingIfImpl>;
    } else if #[cfg(target_arch = "aarch64")]{
        /// The architecture-specific page table.
        pub type GuestPageTableStub = guest_page_table::aarch64::A64PageTable<PagingIfImpl>;
    }
}

#[cfg(feature = "hv")]
pub struct GuestPageTable(GuestPageTableStub);

#[cfg(feature = "hv")]
impl GuestMemoryInterface for GuestPageTable {
    fn new() -> GuestPageTableResult<Self> {
        let npt = GuestPageTableStub::try_new_gpt().map_err(|_| GuestPageTableError::NoMemory)?;
        Ok(GuestPageTable(npt))
    }

    fn map(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: HostPhysAddr,
        flags: MappingFlags,
    ) -> GuestPageTableResult<()> {
        self.0
            .map(
                VirtAddr::from(gpa),
                PhysAddr::from(hpa),
                page_table::PageSize::Size4K,
                flags,
            )
            .map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                GuestPageTableError::Internal
            })?;
        Ok(())
    }

    fn map_region(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: HostPhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> GuestPageTableResult<()> {
        self.0
            .map_region(VirtAddr::from(gpa), PhysAddr::from(hpa), size, flags, true)
            .map_err(|err| {
                error!("paging error: {:?}", err);
                GuestPageTableError::Internal
            })?;
        Ok(())
    }

    fn unmap(&mut self, gpa: GuestPhysAddr) -> GuestPageTableResult<()> {
        #[cfg(target_arch = "riscv64")]
        {
            let (_, _) = self.0.unmap(VirtAddr::from(gpa)).map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                return GuestPageTableError::Internal;
            })?;
            Ok(())
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn translate(&self, gpa: GuestPhysAddr) -> GuestPageTableResult<HostPhysAddr> {
        let (addr, _, _) = self.0.query(VirtAddr::from(gpa)).map_err(|paging_err| {
            error!("paging error: {:?}", paging_err);
            GuestPageTableError::Internal
        })?;
        Ok(addr.into())
    }

    fn token(&self) -> usize {
        // 这个应该和架构是强相关的
        8usize << 60 | usize::from(self.0.root_paddr()) >> 12
    }
}
