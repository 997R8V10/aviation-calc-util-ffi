use aviation_calc_util::{atmos::grib::GribDataPoint, geo::{Longitude, Latitude, GeoPoint, Bearing}, units::{Length, Pressure, Temperature, Velocity}};

crate::ffi_impl_drop_for_struct!(GribDataPoint, atmos_grib_grib_data_point);
crate::ffi_impl_clone_for_struct!(GribDataPoint, atmos_grib_grib_data_point);
crate::ffi_impl_eq_for_struct!(GribDataPoint, atmos_grib_grib_data_point);
crate::ffi_impl_default_for_struct!(GribDataPoint, atmos_grib_grib_data_point);
crate::ffi_impl_display_for_struct!(GribDataPoint, atmos_grib_grib_data_point);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, lat, Latitude);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, lat, Latitude);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, lon, Longitude);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, lon, Longitude);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, geo_pot_height, Length);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, geo_pot_height, Length);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, level_pressure, Pressure);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, level_pressure, Pressure);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, temp, Temperature);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, temp, Temperature);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, v, Velocity);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, v, Velocity);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, u, Velocity);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, u, Velocity);
crate::ffi_gen_get_primitive_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, rh, f64);
crate::ffi_gen_set_primitive_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, rh, f64);
crate::ffi_gen_get_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, sfc_press, Pressure);
crate::ffi_gen_set_struct_var_for_struct!(GribDataPoint, atmos_grib_grib_data_point, sfc_press, Pressure);

#[no_mangle]
pub unsafe extern "C" fn atmos_grib_grib_data_point_new(lat: *mut Latitude, lon: *mut Longitude, level_pressure: *mut Pressure) -> *mut GribDataPoint {
    return Box::leak(Box::new(GribDataPoint::new(*lat, *lon, *level_pressure)));
}

#[no_mangle]
pub unsafe extern "C" fn atmos_grib_grib_data_point_distance_from(ptr: *mut GribDataPoint, point: *mut GeoPoint) -> *mut Length {
    return Box::leak(Box::new((&*ptr).distance_from(&*point)));
}

#[repr(C, packed)]
pub struct InteropWindStruct {
    pub dir: *mut Bearing,
    pub spd: *mut Velocity,
}

#[no_mangle]
pub unsafe extern "C" fn atmos_grib_grib_data_point_wind(ptr: *mut GribDataPoint) -> InteropWindStruct {
    let ret = (&*ptr).wind();
    return InteropWindStruct {
        dir: Box::leak(Box::new(ret.0)),
        spd: Box::leak(Box::new(ret.1))
    };
}