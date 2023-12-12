
use aviation_calc_util::{geo::Longitude, units::Angle};

use crate::interop::{c_str_to_rust_str, rust_str_to_c_str};

use super::DegMinSecStruct;

crate::ffi_impl_drop_for_struct!(Longitude, geo_longitude);
crate::ffi_impl_clone_for_struct!(Longitude, geo_longitude);
crate::ffi_impl_eq_for_struct!(Longitude, geo_longitude);
crate::ffi_impl_default_for_struct!(Longitude, geo_longitude);
crate::ffi_impl_ord_for_struct!(Longitude, geo_longitude);
crate::ffi_impl_display_for_struct!(Longitude, geo_longitude);
crate::ffi_gen_from_method_for_number!(Longitude, geo_longitude, from_radians, f64);
crate::ffi_gen_from_method_for_number!(Longitude, geo_longitude, from_degrees, f64);
crate::ffi_gen_as_method_for_number!(Longitude, geo_longitude, as_radians, f64);
crate::ffi_gen_as_method_for_number!(Longitude, geo_longitude, as_degrees, f64);

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_new(val: *mut Angle) -> *mut Longitude {
    return Box::leak(Box::new(Longitude::new(*val)));
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_as_angle(ptr: *mut Longitude) -> *mut Angle {
    return Box::leak(Box::new((&*ptr).as_angle()));
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_from_deg_min_sec(val: DegMinSecStruct) -> *mut Longitude {
    return Box::leak(Box::new(Longitude::from_deg_min_sec(val.deg, val.min, val.sec)));
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_as_deg_min_sec(ptr: *mut Longitude) -> DegMinSecStruct {
    let dms = (&*ptr).as_deg_min_sec();
    return DegMinSecStruct {
        deg: dms.0,
        min: dms.1,
        sec: dms.2
    };
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_from_vrc(val: *const std::ffi::c_char) -> *mut Longitude {
    return match Longitude::from_vrc(&c_str_to_rust_str(val)) {
        Some(lat) => Box::leak(Box::new(lat)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_from_nats(val: *const std::ffi::c_char) -> *mut Longitude {
    return match Longitude::from_nats(&c_str_to_rust_str(val)) {
        Some(lat) => Box::leak(Box::new(lat)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_as_vrc(ptr: *mut Longitude) -> *const std::ffi::c_char {
    let val = (&*ptr).as_vrc();
    return rust_str_to_c_str(val);
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_as_nats(ptr: *mut Longitude) -> *const std::ffi::c_char {
    let val = (&*ptr).as_nats();
    return rust_str_to_c_str(val);
}

#[no_mangle]
pub unsafe extern "C" fn geo_longitude_normalize_longitude(ptr: *mut Angle) -> *mut Angle {
    return Box::leak(Box::new(Longitude::normalize_longitude(*ptr)));
}