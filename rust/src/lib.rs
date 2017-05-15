#[macro_use]
extern crate mferuby;

#[macro_use]
extern crate lazy_static;

use mferuby::sys;
use mferuby::libc::c_void;
use std::ffi::CString;
use std::mem;

lazy_static! {
  pub static ref poop_type: sys::mrb_data_type = sys::mrb_data_type {
    dtype: cstr!("Poop"),
    dfree: unsafe {mem::transmute(free_poop as *mut c_void)}
  };
}

#[no_mangle]
extern "C" fn free_poop(mrb: *mut sys::mrb_state, map: Box<Poop>) {}

pub struct Poop {
  name: String
}

#[no_mangle]
pub extern "C" fn mrb_poop_hi(mrb: *mut sys::mrb_state, this: sys::mrb_value) -> sys::mrb_value {
  let datap = unsafe {
    sys::mrb_data_get_ptr(mrb, this, &poop_type as &sys::mrb_data_type)
  };

  let it: Poop = unsafe {mem::transmute(datap)};

  println!("name: {}", it.name);

  sys::nil()
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_poop_gem_init(mrb: *mut sys::mrb_state) {
  unsafe {
    let rust_poop = sys::mrb_define_class(mrb, cstr!("Poop"), sys::mrb_state_object_class(mrb));
    sys::MRB::SET_INSTANCE_TT(rust_poop, sys::mrb_vtype::MRB_TT_DATA)
    sys::mrb_define_method(mrb, rust_poop, cstr!("hi"), mrb_poop_hi as sys::mrb_func_t, sys::MRB_ARGS_NONE());
  }
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn mrb_mruby_rust_poop_gem_final(mrb: *mut sys::mrb_state){
}
