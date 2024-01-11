use aviation_calc_util::{geo::{EARTH_RADIUS, EARTH_GRAVITY}, units::{Length, Acceleration}};

pub mod bearing;
pub mod geo_point;
pub mod geo_tile;
pub mod latitude;
pub mod longitude;

#[repr(C)]
pub struct DegMinSecStruct {
    pub deg: i32,
    pub min: u32,
    pub sec: f64
}

crate::ffi_gen_get_struct_const!(EARTH_RADIUS, Length, geo);
crate::ffi_gen_get_struct_const!(EARTH_GRAVITY, Acceleration, geo);