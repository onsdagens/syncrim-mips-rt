use syncrim_mips_rt_macros::text;

fn main() -> ! {
    loop {}
}

#[text]
fn some_fn() -> u32 {
    let a = 32;
    let b = a + 50;
    b
}
