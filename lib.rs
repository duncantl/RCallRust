
#[no_mangle]
pub extern fn bob(val: f64) -> f64 {
  return val* 100.0
}

#[no_mangle]
pub extern fn jane(val: i32) -> i32 {
  return val* 10
}


