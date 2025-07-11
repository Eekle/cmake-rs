#![no_std]
#![allow(warnings)]
pub mod bindings;
pub use bindings::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        bindings::Error_Handler();
    }
    loop {}
}
