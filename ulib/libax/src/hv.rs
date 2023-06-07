//! Hypervisor related functions

pub use axhal::paging::GuestPageTable; 
pub use axruntime::HyperCraftHalImpl;
pub use guest_page_table::{GuestMemoryInterface, GuestPageTableError, GuestPageTableResult};
pub use hypercraft::{PerCpu, VCpu, VmCpus, VM};
pub use hypercraft::VmTrait;


