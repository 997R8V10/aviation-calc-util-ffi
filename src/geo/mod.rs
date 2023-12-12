use aviation_calc_util::{geo::{EARTH_RADIUS, EARTH_GRAVITY}, units::Length};

pub mod bearing;
pub mod geo_point;

crate::ffi_gen_get_struct_const!(EARTH_RADIUS, Length, geo);
crate::ffi_gen_get_primitive_const!(EARTH_GRAVITY, f64, geo);