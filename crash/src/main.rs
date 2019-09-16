fn main() {
    let mut a: usize = 0;
    let ptr: *mut usize = &mut a as *mut usize;

    unsafe {
        // mess up return address of main()
        *ptr.offset(3) = 0x7ffff72f484c;
    }
}
