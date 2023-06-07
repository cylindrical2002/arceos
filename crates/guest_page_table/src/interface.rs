use super::error::GuestPageTableResult;
use page_table_entry::MappingFlags;

/// Guest physical address.
pub type GuestPhysAddr = usize;
/// Guest virtual address.
pub type GuestVirtAddr = usize;
/// Host physical address.
pub type HostPhysAddr = usize;
/// Host virtual address.
pub type HostVirtAddr = usize;
/// Guest page number.
pub type GuestPageNum = usize;
/// Host page number.
pub type HostPageNum = usize;

/// Guest page table trait.
pub trait GuestMemoryInterface {
    /// Create a new guest page table.
    fn new() -> GuestPageTableResult<Self>
    where
        Self: Sized;

    /// Map a guest physical frame starts from `gpa` to the host physical
    /// frame starts from of `hpa` with `flags`.
    fn map(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: HostPhysAddr,
        flags: MappingFlags,
    ) -> GuestPageTableResult<()>;

    /// Map a guest physical region starts from `gpa` to the host physical
    fn map_region(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: HostPhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> GuestPageTableResult<()>;

    /// Unmap the guest physical frame `hpa`
    fn unmap(&mut self, gpa: GuestPhysAddr) -> GuestPageTableResult<()>;

    /// Translate the host physical address which the guest physical frame of
    /// `gpa` maps to.
    fn translate(&self, gpa: GuestPhysAddr) -> GuestPageTableResult<HostPhysAddr>;

    /// Get guest page table token.
    fn token(&self) -> usize;
}
