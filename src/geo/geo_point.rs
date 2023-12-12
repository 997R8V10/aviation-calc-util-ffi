use std::{ops::Sub, ptr};

use aviation_calc_util::{geo::{GeoPoint, Latitude, Longitude, Bearing}, units::Length};


crate::ffi_impl_drop_for_struct!(GeoPoint, geo_geo_point);
crate::ffi_impl_clone_for_struct!(GeoPoint, geo_geo_point);
crate::ffi_impl_eq_for_struct!(GeoPoint, geo_geo_point);
crate::ffi_impl_default_for_struct!(GeoPoint, geo_geo_point);
crate::ffi_gen_get_struct_var_for_struct!(GeoPoint, geo_geo_point, lat, Latitude);
crate::ffi_gen_get_struct_var_for_struct!(GeoPoint, geo_geo_point, lon, Longitude);
crate::ffi_gen_get_struct_var_for_struct!(GeoPoint, geo_geo_point, alt, Length);
crate::ffi_gen_set_struct_var_for_struct!(GeoPoint, geo_geo_point, lat, Latitude);
crate::ffi_gen_set_struct_var_for_struct!(GeoPoint, geo_geo_point, lon, Longitude);
crate::ffi_gen_set_struct_var_for_struct!(GeoPoint, geo_geo_point, alt, Length);

#[no_mangle]
unsafe extern "C" fn geo_geo_point_sub(ptr: *mut GeoPoint, rhs: *mut GeoPoint) -> *mut Length {
    return Box::leak(Box::new((&*ptr).sub(*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_new(lat: *mut Latitude, lon: *mut Longitude, alt: *mut Length) -> *mut GeoPoint {
    return Box::leak(Box::new(GeoPoint::new(*lat, *lon, *alt)));
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_move_by(ptr: *mut GeoPoint, bearing: *mut Bearing, distance: *mut Length) {
    (&mut *ptr).move_by(*bearing, *distance);
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_flat_distance(ptr: *mut GeoPoint, rhs: *mut GeoPoint) -> *mut Length {
    return Box::leak(Box::new((&*ptr).flat_distance(&*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_distance(ptr: *mut GeoPoint, rhs: *mut GeoPoint) -> *mut Length {
    return Box::leak(Box::new((&*ptr).distance(&*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_intersection(ptr: *mut GeoPoint, bearing_1: *mut Bearing, other: *mut GeoPoint, bearing_2: *mut Bearing) -> *mut GeoPoint {
    let result = (&*ptr).intersection(*bearing_1, &*other, *bearing_2);

    return match result {
        Some(point) => Box::leak(Box::new(point)),
        None => ptr::null_mut()
    };
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_closest_intersection(ptr: *mut GeoPoint, bearing_1: *mut Bearing, other: *mut GeoPoint, bearing_2: *mut Bearing) -> *mut GeoPoint {
    let result = (&*ptr).closest_intersection(*bearing_1, &*other, *bearing_2);

    return match result {
        Some(point) => Box::leak(Box::new(point)),
        None => ptr::null_mut()
    };
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_initial_bearing(ptr: *mut GeoPoint, rhs: *mut GeoPoint) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).initial_bearing(&*rhs)));
}

#[no_mangle]
unsafe extern "C" fn geo_geo_point_final_bearing(ptr: *mut GeoPoint, rhs: *mut GeoPoint) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).final_bearing(&*rhs)));
}