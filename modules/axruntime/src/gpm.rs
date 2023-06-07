//! GuestPageTable 向 app 的接口
//! 

use axhal::mem::{PhysAddr, VirtAddr};
use axhal::paging::GuestPagingIfImpl;
use guest_page_table::{
    GuestMemoryInterface, GuestPageTable64Interface, GuestPageTableError, GuestPageTableResult,
    GuestPhysAddr, HostPhysAddr, NestedPageTable,
};
use page_table_entry::MappingFlags;

/// Guest Page Table struct
pub struct GuestPageTable(NestedPageTable<GuestPagingIfImpl>);

impl GuestMemoryInterface for GuestPageTable {
    fn new() -> GuestPageTableResult<Self> {
        #[cfg(target_arch = "riscv64")]
        {
            let npt = NestedPageTable::<GuestPagingIfImpl>::try_new_gpt()
                .map_err(|_| GuestPageTableError::NoMemory)?;
            Ok(GuestPageTable(npt))
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn map(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: HostPhysAddr,
        flags: MappingFlags,
    ) -> GuestPageTableResult<()> {
        #[cfg(target_arch = "riscv64")]
        {
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
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn map_region(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: HostPhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> GuestPageTableResult<()> {
        #[cfg(target_arch = "riscv64")]
        {
            self.0
                .map_region(VirtAddr::from(gpa), PhysAddr::from(hpa), size, flags, true)
                .map_err(|err| {
                    error!("paging error: {:?}", err);
                    GuestPageTableError::Internal
                })?;
            Ok(())
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
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
        #[cfg(target_arch = "riscv64")]
        {
            let (addr, _, _) = self.0.query(VirtAddr::from(gpa)).map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                GuestPageTableError::Internal
            })?;
            Ok(addr.into())
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn token(&self) -> usize {
        #[cfg(target_arch = "riscv64")]
        {
            8usize << 60 | usize::from(self.0.root_paddr()) >> 12
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }
}
