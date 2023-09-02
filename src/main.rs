#![no_std]
#![no_main]

mod writer;
use writer::Writer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start()  {
    let mut writer = Writer::new();
    writer.write("Hey there!!");
    //loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
