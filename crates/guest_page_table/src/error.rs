
/// GuestPageTables Result Define.
/// For interfaces
pub type GuestPageTableResult<T = ()> = Result<T, GuestPageTableError>;

/// The error type for guestpagetable operation failures.
#[derive(Debug, PartialEq)]
pub enum GuestPageTableError {
    /// Internal error.
    Internal,
    /// No supported error.
    NotSupported,
    /// No memory error.
    NoMemory,
}
