use aviation_calc_util::{geo::Latitude, units::Angle};

use crate::interop::{c_str_to_rust_str, rust_str_to_c_str};

use super::DegMinSecStruct;

crate::ffi_impl_drop_for_struct!(Latitude, geo_latitude);
crate::ffi_impl_clone_for_struct!(Latitude, geo_latitude);
crate::ffi_impl_eq_for_struct!(Latitude, geo_latitude);
crate::ffi_impl_default_for_struct!(Latitude, geo_latitude);
crate::ffi_impl_ord_for_struct!(Latitude, geo_latitude);
crate::ffi_impl_display_for_struct!(Latitude, geo_latitude);
crate::ffi_gen_from_method_for_number!(Latitude, geo_latitude, from_radians, f64);
crate::ffi_gen_from_method_for_number!(Latitude, geo_latitude, from_degrees, f64);
crate::ffi_gen_as_method_for_number!(Latitude, geo_latitude, as_radians, f64);
crate::ffi_gen_as_method_for_number!(Latitude, geo_latitude, as_degrees, f64);

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_new(val: *mut Angle) -> *mut Latitude {
    return Box::leak(Box::new(Latitude::new(*val)));
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_as_angle(ptr: *mut Latitude) -> *mut Angle {
    return Box::leak(Box::new((&*ptr).as_angle()));
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_from_deg_min_sec(val: DegMinSecStruct) -> *mut Latitude {
    return Box::leak(Box::new(Latitude::from_deg_min_sec(val.deg, val.min, val.sec)));
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_as_deg_min_sec(ptr: *mut Latitude) -> DegMinSecStruct {
    let dms = (&*ptr).as_deg_min_sec();
    return DegMinSecStruct {
        deg: dms.0,
        min: dms.1,
        sec: dms.2
    };
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_from_vrc(val: *const std::ffi::c_char) -> *mut Latitude {
    return match Latitude::from_vrc(&c_str_to_rust_str(val)) {
        Some(lat) => Box::leak(Box::new(lat)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_from_nats(val: *const std::ffi::c_char) -> *mut Latitude {
    return match Latitude::from_nats(&c_str_to_rust_str(val)) {
        Some(lat) => Box::leak(Box::new(lat)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_as_vrc(ptr: *mut Latitude) -> *const std::ffi::c_char {
    let val = (&*ptr).as_vrc();
    return rust_str_to_c_str(val);
}

#[no_mangle]
pub unsafe extern "C" fn geo_latitude_as_nats(ptr: *mut Latitude) -> *const std::ffi::c_char {
    let val = (&*ptr).as_nats();
    return rust_str_to_c_str(val);
}