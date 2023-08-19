#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use uefi_raw::protocol::loaded_image::LoadedImageProtocol;
use uefi_raw::table::boot::MemoryDescriptor;
use uefi_raw::table::system::SystemTable;
use uefi_raw::{Handle, Status};

use alloc::vec::Vec;
use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr;

mod attribute;
use attribute::OpenProtocolAttributes;

pub mod file_protocol;
use file_protocol::FileProtocol;

mod simple_file_system_protocol;
use simple_file_system_protocol::SimpleFileSystemProtocol;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MemoryMapKey(usize);

struct MemoryMap {
    // buffer_size: u64,
    buffer: *mut MemoryDescriptor,
    map_size: *mut usize,
    map_key: *mut MemoryMapKey,
    descriptor_size: *mut usize,
    descriptor_version: *mut u32,
}

fn GetMemoryMap(map: &MemoryMap, mut system_table: SystemTable) -> Status {
    // if map.buffer == None {
    //     return Status::BUFFER_TOO_SMALL;
    // }
    // map.map_size = Some(map.buffer_size);
    unsafe {
        let bs = system_table.boot_services;

        let status = ((*bs).get_memory_map)(
            map.map_size,
            map.buffer,
            map.map_key.cast::<usize>(),
            map.descriptor_size,
            map.descriptor_version,
        );
        return status;
    }
}

fn OpenRootDir(
    _image_handle: Handle,
    root: &mut *mut FileProtocol,
    mut system_table: SystemTable,
) -> Status {
    let bs = system_table.boot_services;
    let mut loaded_image = ptr::null_mut::<LoadedImageProtocol>();
    let mut fs = ptr::null_mut::<SimpleFileSystemProtocol>();

    unsafe {
        ((*bs).open_protocol)(
            _image_handle,                      // image_handle
            &LoadedImageProtocol::GUID,         // GUID
            &mut loaded_image.cast::<c_void>(), //loaded_image
            _image_handle,                      //NULL
            ptr::null_mut::<c_void>(),
            OpenProtocolAttributes::ByHandleProtocol as u32, // EFI_OPEN
        );
        ((*bs).open_protocol)(
            (*loaded_image).device_handle,   // image_handle
            &SimpleFileSystemProtocol::GUID, // GUID
            &mut fs.cast::<c_void>(),        //loaded_image
            _image_handle,                   //NULL
            ptr::null_mut::<c_void>(),
            OpenProtocolAttributes::ByHandleProtocol as u32, // EFI_OPEN
        );

        ((*fs).open_volume)(fs, root);
    }

    Status::SUCCESS
}

fn main(_image_handle: Handle, mut system_table: SystemTable) -> Status {
    let mut buffer: Vec<u8> = vec![0; 4096 * 4];
    let mut map_size = buffer.len();
    let map_buffer = buffer.as_mut_ptr().cast::<MemoryDescriptor>();
    let mut map_key = MemoryMapKey(0);
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;

    let memmap = MemoryMap {
        buffer: map_buffer,
        map_size: &mut map_size,
        map_key: &mut map_key,
        descriptor_size: &mut descriptor_size,
        descriptor_version: &mut descriptor_version,
    };
    GetMemoryMap(&memmap, system_table);

    // let root = ptr::null_mut::<FileProtocol>();

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
