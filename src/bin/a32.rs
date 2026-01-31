#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub fn entry() {
    naked();
    globalasm();
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
extern "C" fn naked() {
    core::arch::naked_asm!(
        "nop",
        "bx lr",
    );
}

#[instruction_set(arm::a32)]
extern "C" fn globalasm() {
    unsafe { core::arch::asm!("nop"); }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
