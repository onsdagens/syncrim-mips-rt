use syncrim_mips_rt_macros::*;

fn main() -> ! {
    loop {}
}
mod SOME {}
mod SOME {}
#[text]
fn some_fn() -> u32 {
    // some
    let a = 1;
    let b = a + 2;
    b
}

#[text]
fn some_other_fn() -> u32 {
    // some other
    let a = 3;
    let b = a + 4;
    b
}

#[ktext]
fn some_kernel_fn() -> u32 {
    // some kernel
    let a = 5;
    let b = a + 6;
    b
}

#[ktext]
fn some_other_kernel_fn() -> u32 {
    // some other kernel
    let a = 7;
    let b = a + 8;
    b
}

#[data]
static STATIC: u8 = 1;
#[data]
static mut STATIC_MUT: u32 = 1;
#[data]
pub static PUB_STATIC: usize = 1 + 2;
#[data]
pub static mut PUB_STATIC_MUT: bool = true;



#[data]
const CONST: u32 = 1;
#[data]
pub const PUB_CONST: bool = true;
