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

crate::ffi_impl_all_for_number!(Velocity, units_velocity);
crate::ffi_gen_from_method_for_number!(Velocity, units_velocity, from_meters_per_second, f64);
crate::ffi_gen_from_method_for_number!(Velocity, units_velocity, from_knots, f64);
crate::ffi_gen_from_method_for_number!(Velocity, units_velocity, from_feet_per_minute, f64);
crate::ffi_gen_as_method_for_number!(Velocity, units_velocity, as_meters_per_second, f64);
crate::ffi_gen_as_method_for_number!(Velocity, units_velocity, as_knots, f64);
crate::ffi_gen_as_method_for_number!(Velocity, units_velocity, as_feet_per_minute, f64);
crate::ffi_gen_get_primitive_const_for_struct!(Velocity, CONV_FACTOR_MPERS_KTS, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(Velocity, convert_mpers_to_kts, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(Velocity, convert_kts_to_mpers, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(Velocity, convert_mpers_to_ftpermin, f64, units_velocity);
crate::ffi_gen_unit_conv_func!(Velocity, convert_ftpermin_to_mpers, f64, units_velocity);

#[no_mangle]
pub unsafe extern "C" fn units_velocity_mul_duration(ptr: *mut Velocity, rhs: InteropDateTimeStruct) -> *mut Length {
    return Box::leak(Box::new((&*ptr).mul(struct_to_rust_duration(rhs))));
}