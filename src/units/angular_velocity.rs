use aviation_calc_util::units::angular_velocity::AngularVelocity;
use aviation_calc_util::units::unit::Unit;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

crate::ffi_impl_all_for_number!(AngularVelocity, units_angular_velocity);
crate::ffi_gen_from_method_for_number!(AngularVelocity, units_angular_velocity, from_degrees_per_second, f64);
crate::ffi_gen_as_method_for_number!(AngularVelocity, units_angular_velocity, as_radians_per_second, f64);
crate::ffi_gen_as_method_for_number!(AngularVelocity, units_angular_velocity, as_degrees_per_second, f64);
