use core::arch::asm;

#[inline(always)]
pub fn clock() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {output}, %clock;",
            output = out(reg32) result,
        );
    }
    result
}

#[inline(always)]
pub fn thread_idx_x() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %tid.x;",
            r = out(reg32) result,
        );
    }
    result
}

#[inline(always)]
pub fn block_idx_x() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %ctaid.x;",
            r = out(reg32) result,
        );
    }
    result
}

#[panic_handler]
pub unsafe fn breakpoint_panic_handler(_: &::core::panic::PanicInfo) -> ! {
    core::intrinsics::breakpoint();
    core::hint::unreachable_unchecked();
}   
