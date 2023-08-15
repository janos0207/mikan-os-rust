use core::ffi::c_void;
use uefi_raw::{Event, Status};
use uguid::Guid;

type Char16 = u16;

pub struct FileProtocol {
    pub revision: u64,
    pub open: unsafe extern "efiapi" fn(
        this: *mut Self,
        new_handle: &mut *mut Self,
        file_name: *const Char16, // correct?
        open_mode: u64,
        attributes: u64,
    ) -> Status,
    pub close: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub delete: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub read: unsafe extern "efiapi" fn(
        this: *mut Self,
        buffer_size: &mut usize,
        buffer: *mut u8,
    ) -> Status,
    pub write: unsafe extern "efiapi" fn(
        this: *mut Self,
        buffer_size: &mut usize,
        buffer: *const u8,
    ) -> Status,
    pub get_position: extern "efiapi" fn(this: *mut Self, position: *mut u64) -> Status,
    pub set_position: extern "efiapi" fn(this: *mut Self, position: u64) -> Status,
    pub get_info: extern "efiapi" fn(
        this: *mut Self,
        information_type: &Guid,
        buffer_size: &mut usize,
        buffer: *const c_void,
    ) -> Status,
    pub set_info: extern "efiapi" fn(
        this: *mut Self,
        information_type: &Guid,
        buffer_size: &mut usize,
        buffer: *const c_void,
    ),
    pub flush: extern "efiapi" fn(this: *mut Self, token: *mut FileIoToken) -> Status,
}

struct FileIoToken {
    pub event: Event,
    pub status: Status,
    pub buffer_size: usize,
    pub buffer: *const c_void,
}
