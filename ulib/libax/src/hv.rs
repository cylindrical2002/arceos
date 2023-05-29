//! Hypervisor related functions

pub use axruntime::{GuestPageTable, HyperCraftHalImpl};
pub use hypercraft::GuestPageTableTrait;

pub use hypercraft::HyperError as Error;
pub use hypercraft::HyperResult as Result;
pub use hypercraft::{HyperCallMsg, PerCpu, VCpu, VmCpus, VmExitInfo, VM};

pub use hypercraft::VmTrait;


