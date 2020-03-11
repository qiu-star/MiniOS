
global_asm!(include_str!("boot/entry.asm")); // 引入 _start


#[no_mangle]
pub fn rust_main() -> ! {
    println!("{}","hello world");
    // put_string("h");
    loop {}
}