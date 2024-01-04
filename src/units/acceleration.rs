use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

use aviation_calc_util::units::Acceleration;
use aviation_calc_util::units::Velocity;
use aviation_calc_util::units::Unit;

use crate::interop::InteropDateTimeStruct;
use crate::interop::struct_to_rust_duration;

crate::ffi_impl_all_for_number!(Acceleration, units_acceleration);
crate::ffi_gen_from_method_for_number!(Acceleration, units_acceleration, from_meters_per_second_squared, f64);
crate::ffi_gen_from_method_for_number!(Acceleration, units_acceleration, from_knots_per_second, f64);
crate::ffi_gen_as_method_for_number!(Acceleration, units_acceleration, as_meters_per_second_squared, f64);
crate::ffi_gen_as_method_for_number!(Acceleration, units_acceleration, as_knots_per_second, f64);

#[no_mangle]
pub unsafe extern "C" fn units_acceleration_mul_duration(ptr: *mut Acceleration, rhs: InteropDateTimeStruct) -> *mut Velocity {
    return Box::leak(Box::new((&*ptr).mul(struct_to_rust_duration(rhs))));
}