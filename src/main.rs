#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let string = cstr16!("Hello world!\n");
    system_table.stdout().output_string(string).unwrap();
    loop {}
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
