use aviation_calc_util::units::angle::Angle;
use aviation_calc_util::units::angular_velocity::AngularVelocity;
use aviation_calc_util::units::length::Length;
use aviation_calc_util::units::length::CONV_FACTOR_MI_M;
use aviation_calc_util::units::length::CONV_FACTOR_M_FT;
use aviation_calc_util::units::length::CONV_FACTOR_NMI_M;
use aviation_calc_util::units::length::convert_feet_to_meters;
use aviation_calc_util::units::length::convert_meters_to_feet;
use aviation_calc_util::units::length::convert_meters_to_nautical_miles;
use aviation_calc_util::units::length::convert_meters_to_statute_miles;
use aviation_calc_util::units::length::convert_nautical_miles_to_meters;
use aviation_calc_util::units::length::convert_statute_miles_to_meters;
use aviation_calc_util::units::pressure::Pressure;
use aviation_calc_util::units::temperature::Temperature;
use aviation_calc_util::units::velocity::Velocity;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::ffi_gen_as_method_for_number;
use crate::ffi_gen_from_method_for_number;
use crate::ffi_impl_all_for_number;

// Length
ffi_impl_all_for_number!(Length, units_length);
ffi_gen_from_method_for_number!(Length, units_length, from_feet, f64);
ffi_gen_from_method_for_number!(Length, units_length, from_nautical_miles, f64);
ffi_gen_from_method_for_number!(Length, units_length, from_statute_miles, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_meters, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_feet, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_nautical_miles, f64);
ffi_gen_as_method_for_number!(Length, units_length, as_statute_miles, f64);
crate::ffi_gen_get_primitive_const!(CONV_FACTOR_M_FT, f64, units_length);
crate::ffi_gen_get_primitive_const!(CONV_FACTOR_NMI_M, f64, units_length);
crate::ffi_gen_get_primitive_const!(CONV_FACTOR_MI_M, f64, units_length);
crate::ffi_gen_unit_conv_func!(convert_meters_to_feet, f64, units_length);
crate::ffi_gen_unit_conv_func!(convert_meters_to_nautical_miles, f64, units_length);
crate::ffi_gen_unit_conv_func!(convert_meters_to_statute_miles, f64, units_length);
crate::ffi_gen_unit_conv_func!(convert_feet_to_meters, f64, units_length);
crate::ffi_gen_unit_conv_func!(convert_nautical_miles_to_meters, f64, units_length);
crate::ffi_gen_unit_conv_func!(convert_statute_miles_to_meters, f64, units_length);

// Pressure
ffi_impl_all_for_number!(Pressure, units_pressure);
ffi_gen_from_method_for_number!(Pressure, units_pressure, from_hectopascals, f64);
ffi_gen_from_method_for_number!(Pressure, units_pressure, from_inches_of_mercury, f64);
ffi_gen_as_method_for_number!(Pressure, units_pressure, as_pascals, f64);
ffi_gen_as_method_for_number!(Pressure, units_pressure, as_inches_of_mercury, f64);
ffi_gen_as_method_for_number!(Pressure, units_pressure, as_hectopascals, f64);

// Temperature
ffi_impl_all_for_number!(Temperature, units_temperature);
ffi_gen_from_method_for_number!(Temperature, units_temperature, from_celsius, f64);
ffi_gen_as_method_for_number!(Temperature, units_temperature, as_kelvin, f64);
ffi_gen_as_method_for_number!(Temperature, units_temperature, as_celsius, f64);

// Velocity
ffi_impl_all_for_number!(Velocity, units_velocity);
ffi_gen_from_method_for_number!(Velocity, units_velocity, from_knots, f64);
ffi_gen_from_method_for_number!(Velocity, units_velocity, from_feet_per_minute, f64);
ffi_gen_as_method_for_number!(Velocity, units_velocity, as_meters_per_second, f64);
ffi_gen_as_method_for_number!(Velocity, units_velocity, as_knots, f64);
ffi_gen_as_method_for_number!(Velocity, units_velocity, as_feet_per_minute, f64);

// Angle
ffi_impl_all_for_number!(Angle, units_angle);
ffi_gen_from_method_for_number!(Angle, units_angle, from_degrees, f64);
ffi_gen_as_method_for_number!(Angle, units_angle, as_radians, f64);
ffi_gen_as_method_for_number!(Angle, units_angle, as_degrees, f64);

// Angular Velocity
ffi_impl_all_for_number!(AngularVelocity, units_angular_velocity);
ffi_gen_from_method_for_number!(AngularVelocity, units_angular_velocity, from_degrees_per_second, f64);
ffi_gen_as_method_for_number!(AngularVelocity, units_angular_velocity, as_radians_per_second, f64);
ffi_gen_as_method_for_number!(AngularVelocity, units_angular_velocity, as_degrees_per_second, f64);
