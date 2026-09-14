#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let x = b"Hello, World!";
    let os = 0xb8000 as *mut u8;
    for (i, byte) in x.iter().enumerate() {
        unsafe {

    *os.add(i * 2) = *byte;
    *os.add(i * 2 + 1) = 0x0f;
}}
loop{}
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}