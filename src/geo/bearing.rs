use std::ops::{Neg, Add, Sub};

use aviation_calc_util::{geo::Bearing, units::Angle};


crate::ffi_impl_drop_for_struct!(Bearing, geo_bearing);
crate::ffi_impl_clone_for_struct!(Bearing, geo_bearing);
crate::ffi_impl_eq_for_struct!(Bearing, geo_bearing);
crate::ffi_impl_default_for_struct!(Bearing, geo_bearing);
crate::ffi_impl_ord_for_struct!(Bearing, geo_bearing);
crate::ffi_impl_display_for_struct!(Bearing, geo_bearing);
crate::ffi_gen_from_method_for_number!(Bearing, geo_bearing, from_radians, f64);
crate::ffi_gen_from_method_for_number!(Bearing, geo_bearing, from_degrees, f64);
crate::ffi_gen_as_method_for_number!(Bearing, geo_bearing, as_radians, f64);
crate::ffi_gen_as_method_for_number!(Bearing, geo_bearing, as_degrees, f64);

#[no_mangle]
unsafe extern "C" fn geo_bearing_new(val: *mut Angle) -> *mut Bearing {
    return Box::leak(Box::new(Bearing::new(*val)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_as_angle(ptr: *mut Bearing) -> *mut Angle {
    return Box::leak(Box::new((&*ptr).as_angle()));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_neg(ptr: *mut Bearing) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).neg()));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_add_angle(ptr: *mut Bearing, rhs: *mut Angle) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).add(*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_sub(ptr: *mut Bearing, rhs: *mut Bearing) -> *mut Angle {
    return Box::leak(Box::new((&*ptr).sub(*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_sub_angle(ptr: *mut Bearing, rhs: *mut Angle) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).sub(*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_normalize_bearing(ptr: *mut Angle) -> *mut Angle {
    return Box::leak(Box::new(Bearing::normalize_bearing(*ptr)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_calculate_bearing_delta(bearing_1: *mut Bearing, bearing_2: *mut Bearing) -> *mut Angle {
    return Box::leak(Box::new(Bearing::calculate_bearing_delta(*bearing_1, *bearing_2)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_calculate_bearing_delta_with_dir(bearing_1: *mut Bearing, bearing_2: *mut Bearing, is_right_turn: bool) -> *mut Angle {
    return Box::leak(Box::new(Bearing::calculate_bearing_delta_with_dir(*bearing_1, *bearing_2, is_right_turn)));
}

#[no_mangle]
unsafe extern "C" fn geo_bearing_calculate_end_bearing(start_bearing: *mut Bearing, bearing_delta: *mut Angle) -> *mut Bearing {
    return Box::leak(Box::new(Bearing::calculate_end_bearing(*start_bearing, *bearing_delta)));
}