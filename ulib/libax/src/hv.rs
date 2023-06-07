//! Hypervisor related functions

pub use axruntime::{GuestPageTable, HyperCraftHalImpl};
pub use guest_page_table::{GuestMemoryInterface, GuestPageTableError, GuestPageTableResult};
pub use hypercraft::{HyperCallMsg, PerCpu, VCpu, VmCpus, VmExitInfo, VM};
pub use hypercraft::VmTrait;


