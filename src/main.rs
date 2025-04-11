
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct TEEC_Context {
    pub imp: TEEC_Context__bindgen_ty_1,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct TEEC_Context__bindgen_ty_1 {
    pub fd: ::std::os::raw::c_int,
    pub reg_mem: bool,
    pub memref_null: bool,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct TEEC_Session {
    //pub imp: TEEC_Session__bindgen_ty_1,
    pub ctx: *mut TEEC_Context,
    pub session_id: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct TEEC_Session__bindgen_ty_1 {
    pub ctx: *mut TEEC_Context,
    pub session_id: u32,
}

fn main() {
    type U = TEEC_Session;

    use std::mem::{MaybeUninit, size_of};
    let mut x = MaybeUninit::<U>::uninit();
    let xptr = x.as_mut_ptr().cast::<u8>();
    let size = size_of::<U>();

    for i in 0..size {
        unsafe{ xptr.add(i).write_volatile(i as u8); }
    }
    unsafe{ print_memory(size as u32, &mut x.assume_init()) };
}

unsafe extern "C" {
    fn print_memory(size: u32, mem: *mut TEEC_Session);
}
