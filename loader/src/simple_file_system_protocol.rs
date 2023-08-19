use super::file_protocol::FileProtocol;
use uefi_raw::Status;
use uguid::{guid, Guid};

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision: u64,
    pub open_volume:
        unsafe extern "efiapi" fn(this: *mut Self, root: &mut *mut FileProtocol) -> Status,
}

impl SimpleFileSystemProtocol {
    pub const GUID: Guid = guid!("5b1b31a1-9562-11d2-8e3f-00a0c969723b");
}
