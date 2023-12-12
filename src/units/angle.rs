use aviation_calc_util::units::angle::Angle;
use aviation_calc_util::units::unit::Unit;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

crate::ffi_impl_all_for_number!(Angle, units_angle);
crate::ffi_gen_from_method_for_number!(Angle, units_angle, from_degrees, f64);
crate::ffi_gen_as_method_for_number!(Angle, units_angle, as_radians, f64);
crate::ffi_gen_as_method_for_number!(Angle, units_angle, as_degrees, f64);
