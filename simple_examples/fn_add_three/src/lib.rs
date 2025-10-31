use prudent::unsafe_fn;

unsafe fn add_three(left: u64, middle:u64, right: u64) -> u64 {
    left + middle + right
}

pub fn add() {
    unsafe_fn!(add_three, 1, 2, 3);
}

