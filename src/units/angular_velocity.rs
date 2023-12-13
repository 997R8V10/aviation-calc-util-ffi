use aviation_calc_util::units::Angle;
use aviation_calc_util::units::AngularVelocity;
use aviation_calc_util::units::Unit;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

use crate::interop::InteropDateTimeStruct;
use crate::interop::struct_to_rust_duration;

crate::ffi_impl_all_for_number!(AngularVelocity, units_angular_velocity);
crate::ffi_gen_from_method_for_number!(AngularVelocity, units_angular_velocity, from_radians_per_second, f64);
crate::ffi_gen_from_method_for_number!(AngularVelocity, units_angular_velocity, from_degrees_per_second, f64);
crate::ffi_gen_as_method_for_number!(AngularVelocity, units_angular_velocity, as_radians_per_second, f64);
crate::ffi_gen_as_method_for_number!(AngularVelocity, units_angular_velocity, as_degrees_per_second, f64);

#[no_mangle]
pub unsafe extern "C" fn units_angular_velocity_mul_duration(ptr: *mut AngularVelocity, rhs: InteropDateTimeStruct) -> *mut Angle {
    return Box::leak(Box::new((&*ptr).mul(struct_to_rust_duration(rhs))));
}