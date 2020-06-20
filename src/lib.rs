#![no_std]
#![feature(lang_items)]

extern "C" {
    #[no_mangle]
    fn printk(s: *const u8, len: cty::c_int) -> cty::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn rust_module_start() -> cty::c_int {
    printk("hello rust!\n\0".as_bytes().as_ptr(), 13);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn rust_module_end() {
    printk("bye rust!\n\0".as_bytes().as_ptr(), 11);
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
