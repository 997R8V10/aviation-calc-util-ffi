use aviation_calc_util::units::Length;
use aviation_calc_util::units::Unit;
use aviation_calc_util::units::Velocity;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

use crate::interop::InteropDateTimeStruct;
use crate::interop::struct_to_rust_duration;

crate::ffi_impl_all_for_number!(Length, units_length);
crate::ffi_gen_from_method_for_number!(Length, units_length, from_meters, f64);
crate::ffi_gen_from_method_for_number!(Length, units_length, from_feet, f64);
crate::ffi_gen_from_method_for_number!(Length, units_length, from_nautical_miles, f64);
crate::ffi_gen_from_method_for_number!(Length, units_length, from_statute_miles, f64);
crate::ffi_gen_as_method_for_number!(Length, units_length, as_meters, f64);
crate::ffi_gen_as_method_for_number!(Length, units_length, as_feet, f64);
crate::ffi_gen_as_method_for_number!(Length, units_length, as_nautical_miles, f64);
crate::ffi_gen_as_method_for_number!(Length, units_length, as_statute_miles, f64);
crate::ffi_gen_get_primitive_const_for_struct!(Length, CONV_FACTOR_M_FT, f64, units_length);
crate::ffi_gen_get_primitive_const_for_struct!(Length, CONV_FACTOR_NMI_M, f64, units_length);
crate::ffi_gen_get_primitive_const_for_struct!(Length, CONV_FACTOR_MI_M, f64, units_length);
crate::ffi_gen_unit_conv_func!(Length, convert_meters_to_feet, f64, units_length);
crate::ffi_gen_unit_conv_func!(Length, convert_meters_to_nautical_miles, f64, units_length);
crate::ffi_gen_unit_conv_func!(Length, convert_meters_to_statute_miles, f64, units_length);
crate::ffi_gen_unit_conv_func!(Length, convert_feet_to_meters, f64, units_length);
crate::ffi_gen_unit_conv_func!(Length, convert_nautical_miles_to_meters, f64, units_length);
crate::ffi_gen_unit_conv_func!(Length, convert_statute_miles_to_meters, f64, units_length);

#[no_mangle]
pub unsafe extern "C" fn units_length_div_duration(ptr: *mut Length, rhs: InteropDateTimeStruct) -> *mut Velocity {
    return Box::leak(Box::new((&*ptr).div(struct_to_rust_duration(rhs))));
}