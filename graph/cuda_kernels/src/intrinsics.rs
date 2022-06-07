use core::arch::asm;

#[inline(always)]
pub fn thread_idx_x() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %tid.x;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn thread_idx_y() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %tid.y;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn thread_idx_z() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %tid.z;",
            r = out(reg32) result,
        );
    }
    if result > 64 {
        unsafe{core::hint::unreachable_unchecked()};
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
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn block_idx_y() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %ctaid.y;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn block_idx_z() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %ctaid.z;",
            r = out(reg32) result,
        );
    }
    if result > 64 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn block_dim_x() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %ntid.x;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn block_dim_y() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %ntid.y;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn block_dim_z() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %ntid.z;",
            r = out(reg32) result,
        );
    }
    if result > 64 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn grid_dim_x() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %nctaid.x;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn grid_dim_y() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %nctaid.y;",
            r = out(reg32) result,
        );
    }
    if result > 1024 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

#[inline(always)]
pub fn grid_dim_z() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {r}, %nctaid.z;",
            r = out(reg32) result,
        );
    }
    if result > 64 {
        unsafe{core::hint::unreachable_unchecked()};
    }
    result
}

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
pub fn wrap_sz() -> u32{
    let mut result: u32;
    unsafe {
        asm!(
            "mov.u32 {output}, WARP_SZ;",
            output = out(reg32) result,
        );
    }
    result
}


/// A trait that allows us to add methods to primitive types
pub trait FloatsMissingOps {
    /// Compute the square root of the value
    fn sqrt(&self) -> Self;

    /// Compute the 2**value
    fn exp2(&self) -> Self;

    /// Compute log2 of the value
    fn log2(&self) -> Self;

    /// a * b + c but fast
    fn fma(a: Self, b: Self, c: Self) -> Self;
}

/// The actual implementation calling raw PTX assembly
impl FloatsMissingOps for f32 {
    #[inline(always)]
    fn sqrt(&self) -> f32 {
        let mut result: f32;
        unsafe {
            asm!(
                // here we use the fast and approximated sqrt,
                // we could use sqrt.rnd.f32 if we want slower but
                // IEEE 754 compliant rounding
                "sqrt.approx.f32 {output}, {input};",
                input = in(reg32) *self,
                output = out(reg32) result,
            );
        }
        result
    }

    #[inline(always)]
    fn exp2(&self) -> f32 {
        let mut result: f32;
        unsafe {
            asm!(
                // here we use the fast and approximated sqrt,
                // we could use sqrt.rnd.f32 if we want slower but
                // IEEE 754 compliant rounding
                "ex2.approx.f32 {output}, {input};",
                input = in(reg32) *self,
                output = out(reg32) result,
            );
        }
        result
    }

    #[inline(always)]
    fn log2(&self) -> f32 {
        let mut result: f32;
        unsafe {
            asm!(
                // here we use the fast and approximated sqrt,
                // we could use sqrt.rnd.f32 if we want slower but
                // IEEE 754 compliant rounding
                "lg2.approx.f32 {output}, {input};",
                input = in(reg32) *self,
                output = out(reg32) result,
            );
        }
        result
    }

    #[inline(always)]
    fn fma(a: f32, b: f32, c: f32) -> f32 {
        let mut result: f32;
        unsafe {
            asm!(
                // here we use the fast and approximated sqrt,
                // we could use sqrt.rnd.f32 if we want slower but
                // IEEE 754 compliant rounding
                "fma.rn.f32 {output}, {a}, {b}, {c};",
                a = in(reg32) a,
                b = in(reg32) b,
                c = in(reg32) c,
                output = out(reg32) result,
            );
        }
        result
    }
}

#[panic_handler]
pub unsafe fn breakpoint_panic_handler(_: &::core::panic::PanicInfo) -> ! {
    core::intrinsics::breakpoint();
    core::hint::unreachable_unchecked();
}   
