use crate::syscall::sys_exit;
use crate::ALLOCATOR;

use core::alloc::Layout;
use core::panic::PanicInfo;
use super::syscall::*;

#[linkage = "weak"]
#[no_mangle]
fn main() {
    panic!("No main() linked");
}

fn init_heap() {
    const HEAP_SIZE: usize = 0x1000;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { ALLOCATOR.lock().init(HEAP.as_ptr() as usize, HEAP_SIZE); }
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    init_heap();
    main();
    sys_exit(0)
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n\n{}", info);
    sys_exit(1)
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    panic!("abort");
}
