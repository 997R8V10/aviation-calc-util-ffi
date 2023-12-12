use aviation_calc_util::magnetic;

use crate::interop::{struct_to_rust_naive_date, InteropDateStruct};

#[no_mangle]
pub unsafe extern "C" fn magnetic_get_epoch_year(date: InteropDateStruct) -> f64 {
    return magnetic::get_epoch_year(&struct_to_rust_naive_date(date).unwrap());
}

pub mod magnetic_model;
pub mod magnetic_tile;
pub mod magnetic_field;
pub mod magnetic_model_coefficients;
