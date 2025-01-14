use page_table::{PageTable64, PagingMetaData};
use page_table_entry::riscv::Rv64PTE;

pub struct Sv39x4MetaData;

impl PagingMetaData for Sv39x4MetaData {
    const LEVELS: usize = 3;
    const PA_MAX_BITS: usize = 56;
    // G-stage root page table: 16KiB
    const VA_MAX_BITS: usize = 41;
}

/// Nested page table define.
pub type Sv39x4PageTable<I> = PageTable64<Sv39x4MetaData, Rv64PTE, I>;
