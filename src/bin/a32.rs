#![no_std]
#![no_main]

use core::arch::global_asm;

#[unsafe(no_mangle)]
pub fn entry() {
    nakedfn();
    normalfn();
    globalfn();
}

#[unsafe(naked)]
#[instruction_set(arm::a32)]
extern "C" fn nakedfn() {
    core::arch::naked_asm!("nop", "bx lr",);
}

#[instruction_set(arm::a32)]
extern "C" fn normalfn() {
    unsafe {
        core::arch::asm!("nop");
    }
}

unsafe extern "C" {
    safe fn globalfn();
}

global_asm!(
    r#"
    .arm
    .global globalfn
    .type globalfn, %function
    globalfn:
        push    {{ r7, lr }}
        nop
        pop     {{ r7, pc }}
    .size globalfn, . - globalfn
"#
);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
