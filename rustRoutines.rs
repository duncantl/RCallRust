#![crate_type = "dylib"]


#[no_mangle]
pub extern fn bob(val: f64) -> f64 {
  return val* 100.0
}

#[no_mangle]
pub extern fn jane(val: i32) -> i32 {
  return val* 10
}

#[no_mangle]
pub unsafe extern fn rf_jane(val: *mut i32)  {
  *val = jane(*val);
}

#[no_mangle]
pub unsafe extern fn rf_bob(val: *mut f64)  {
  *val = bob(*val);
}