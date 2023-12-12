use aviation_calc_util::{geo::{EARTH_RADIUS, EARTH_GRAVITY}, units::Length};

pub mod bearing;
pub mod geo_point;
pub mod geo_tile;
pub mod latitude;
pub mod longitude;

#[repr(C, packed)]
pub struct DegMinSecStruct {
    pub deg: i32,
    pub min: u32,
    pub sec: f64
}

crate::ffi_gen_get_struct_const!(EARTH_RADIUS, Length, geo);
crate::ffi_gen_get_primitive_const!(EARTH_GRAVITY, f64, geo);