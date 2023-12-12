use aviation_calc_util::units::Temperature;
use aviation_calc_util::units::Unit;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

crate::ffi_impl_all_for_number!(Temperature, units_temperature);
crate::ffi_gen_from_method_for_number!(Temperature, units_temperature, from_kelvin, f64);
crate::ffi_gen_from_method_for_number!(Temperature, units_temperature, from_celsius, f64);
crate::ffi_gen_as_method_for_number!(Temperature, units_temperature, as_kelvin, f64);
crate::ffi_gen_as_method_for_number!(Temperature, units_temperature, as_celsius, f64);
crate::ffi_gen_get_primitive_const_for_struct!(Temperature, CONV_FACTOR_KELVIN_C, f64, units_temperature);
crate::ffi_gen_unit_conv_func!(Temperature, convert_kelvin_to_celsius, f64, units_temperature);
crate::ffi_gen_unit_conv_func!(Temperature, convert_celsius_to_kelvin, f64, units_temperature);
