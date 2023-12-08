use crate::ffi_impl_all_for_number;
use crate::ffi_gen_from_method_for_number;
use crate::ffi_gen_as_method_for_number;

// Length
use aviation_calc_util::units::length::Length;
ffi_impl_all_for_number!(Length, units_length);
ffi_gen_from_method_for_number!(Length, units_length, from_feet, f64);
ffi_gen_from_method_for_number!(Length, units_length, from_nautical_miles, f64);
ffi_gen_from_method_for_number!(Length, units_length, from_statute_miles, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_meters, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_feet, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_nautical_miles, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_statute_miles, f64);