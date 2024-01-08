use aviation_calc_util::units::Angle;
use aviation_calc_util::units::AngularVelocity;
use aviation_calc_util::units::Length;
use aviation_calc_util::units::Unit;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::Sub;

use crate::geo::DegMinSecStruct;
use crate::interop::InteropDateTimeStruct;
use crate::interop::struct_to_rust_duration;

crate::ffi_impl_all_for_number!(Angle, units_angle);
crate::ffi_gen_from_method_for_number!(Angle, units_angle, from_radians, f64);
crate::ffi_gen_from_method_for_number!(Angle, units_angle, from_degrees, f64);
crate::ffi_gen_from_method_for_number!(Angle, units_angle, from_percentage, f64);
crate::ffi_gen_as_method_for_number!(Angle, units_angle, as_radians, f64);
crate::ffi_gen_as_method_for_number!(Angle, units_angle, as_degrees, f64);
crate::ffi_gen_as_method_for_number!(Angle, units_angle, as_percentage, f64);

#[no_mangle]
pub unsafe extern "C" fn units_angle_div_duration(ptr: *mut Angle, rhs: InteropDateTimeStruct) -> *mut AngularVelocity {
    return Box::leak(Box::new((&*ptr).div(struct_to_rust_duration(rhs))));
}

#[no_mangle]
pub unsafe extern "C" fn units_angle_from_deg_min_sec(val: DegMinSecStruct) -> *mut Angle {
    return Box::leak(Box::new(Angle::from_deg_min_sec(val.deg, val.min, val.sec)));
}

#[no_mangle]
pub unsafe extern "C" fn units_angle_as_deg_min_sec(ptr: *mut Angle) -> DegMinSecStruct {
    let dms = (&*ptr).as_deg_min_sec();
    return DegMinSecStruct {
        deg: dms.0,
        min: dms.1,
        sec: dms.2
    };
}

#[no_mangle]
pub unsafe extern "C" fn units_angle_calculate_angle_from_arc_length(arc_length: *mut Length, radius: *mut Length) -> *mut Angle {
    return Box::leak(Box::new(Angle::calculate_angle_from_arc_length(*arc_length, *radius)));
}

#[no_mangle]
pub unsafe extern "C" fn units_angle_calculate_arc_length(angle: *mut Angle, radius: *mut Length) -> *mut Length {
    return Box::leak(Box::new(Angle::calculate_arc_length(*angle, *radius)));
}