static mut COUNTER: u32 = 0;

unsafe fn add_to_count(inc: u32) {
    COUNTER += inc;
}

fn main() {
    let x: u32 = 10;
    unsafe { add_to_count(x); }
}