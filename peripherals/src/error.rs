//! Error definition for memory mapped IO.

#[derive(Debug, Fail, PartialEq)]
pub enum MemoryAccessError {
    #[fail(display = "device not mapped at {}", addr)]
    DeviceNotMapped { addr: usize },

    #[fail(display = "permission error")]
    NoPermission,

    #[fail(display = "should align {} byte alignment", alignment)]
    InvalidAlignment { alignment: usize },

    #[fail(display = "cannot read enough length of data from memory")]
    UnexpectedEom,
}
