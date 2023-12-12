use aviation_calc_util::units::unit::Unit;
use aviation_calc_util::units::velocity::convert_ftpermin_to_mpers;
use aviation_calc_util::units::velocity::convert_kts_to_mpers;
use aviation_calc_util::units::velocity::convert_mpers_to_ftpermin;
use aviation_calc_util::units::velocity::convert_mpers_to_kts;
use aviation_calc_util::units::velocity::Velocity;
use aviation_calc_util::units::velocity::CONV_FACTOR_MPERS_KTS;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

crate::ffi_impl_all_for_number!(Velocity, units_velocity);
crate::ffi_gen_from_method_for_number!(Velocity, units_velocity, from_knots, f64);
crate::ffi_gen_from_method_for_number!(Velocity, units_velocity, from_feet_per_minute, f64);
crate::ffi_gen_as_method_for_number!(Velocity, units_velocity, as_meters_per_second, f64);
crate::ffi_gen_as_method_for_number!(Velocity, units_velocity, as_knots, f64);
crate::ffi_gen_as_method_for_number!(Velocity, units_velocity, as_feet_per_minute, f64);
crate::ffi_gen_get_primitive_const!(CONV_FACTOR_MPERS_KTS, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(convert_mpers_to_kts, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(convert_kts_to_mpers, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(convert_mpers_to_ftpermin, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(convert_ftpermin_to_mpers, f64, units_velocity);
