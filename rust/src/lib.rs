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

pub struct Poop<'a> {
  name: &'a str
}

#[no_mangle]
pub extern "C" fn mrb_poop_init(mrb: *mut sys::mrb_state, this: sys::mrb_value) -> sys::mrb_value {
  let datap = unsafe {
    sys::mrb_data_get_ptr(mrb, this, &poop_type as &sys::mrb_data_type)
  };

  let mut name: sys::mrb_value = unsafe {mem::uninitialized()};

  unsafe {
    sys::mrb_get_args(mrb, cstr!("S"), &mut name);
  }

  let rname = mferuby::mruby_str_to_rust_string(name).unwrap();

  println!("mrb_poop_init got name: {}", rname);

  let rdata = &Poop {name: rdata.as_str()};

  let mrb_obj = mferuby::Mrb::new(mrb);
  let mybox = Box::new(rdata);
  let klass = unsafe {sys::mrb_class_get(mrb, cstr!("Poop"))};
  unsafe {sys::mrb_obj_value(mrb_obj.data_object_alloc::<Poop>(klass, mybox, &poop_type))}

}

#[no_mangle]
pub extern "C" fn mrb_poop_hi(mrb: *mut sys::mrb_state, this: sys::mrb_value) -> sys::mrb_value {
  let datap = unsafe {
    sys::mrb_data_get_ptr(mrb, this, &poop_type as &sys::mrb_data_type)
  };

  let it: &Poop = unsafe {mem::transmute(datap)};

  println!("name: {}", it.name);

  unsafe {sys::nil()}
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_poop_gem_init(mrb: *mut sys::mrb_state) {
  unsafe {
    let rust_poop = sys::mrb_define_class(mrb, cstr!("Poop"), sys::mrb_state_object_class(mrb));
    sys::MRB_SET_INSTANCE_TT(rust_poop, sys::mrb_vtype::MRB_TT_DATA);
    sys::mrb_define_method(mrb, rust_poop, cstr!("initialize"), mrb_poop_init as sys::mrb_func_t, sys::MRB_ARGS_REQ(1));
    sys::mrb_define_method(mrb, rust_poop, cstr!("hi"), mrb_poop_hi as sys::mrb_func_t, sys::MRB_ARGS_NONE());
  }
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn mrb_mruby_rust_poop_gem_final(mrb: *mut sys::mrb_state){
}
