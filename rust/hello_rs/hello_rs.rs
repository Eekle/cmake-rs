#![no_std]

#[unsafe(no_mangle)]
pub extern "C" fn hello_from_rust_loop() {
    loop {
        let message = "Hello from Rust!\n".as_bytes();
        unsafe {
            bindings::HAL_UART_Transmit(
                &raw mut bindings::huart2,
                message.as_ptr(),
                message.len() as u16,
                bindings::HAL_MAX_DELAY,
            );
            bindings::HAL_Delay(1000);
        }
    }
}
