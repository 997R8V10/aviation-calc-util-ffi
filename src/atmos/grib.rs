use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use aviation_calc_util::{
    atmos::grib::{GribDataPoint, GribTile, GribTileManager},
    geo::{GeoPoint, GeoTile},
};

use crate::interop::{c_str_to_rust_str, rust_date_time_to_struct, rust_str_to_c_str, struct_to_rust_date_time, InteropDateTimeStruct};

// GribTileManager
crate::ffi_impl_drop_for_struct!(GribTileManager, grib_grib_tile_manager);

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_manager_new(download_path: *const std::ffi::c_char) -> *mut GribTileManager {
    let path_buf = PathBuf::new().join(c_str_to_rust_str(download_path));
    return Box::leak(Box::new(GribTileManager::new(&path_buf)));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_manager_find_or_create_tile(ptr: *mut GribTileManager, point: *mut GeoPoint, date: InteropDateTimeStruct) -> *mut Arc<Mutex<GribTile>> {
    let arc_tile = (&*ptr).find_or_create_tile(&*point, &struct_to_rust_date_time(date).unwrap());
    return Box::leak(Box::new(arc_tile));
}

// Arc<GribTile>
#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_drop(ptr: *mut Arc<Mutex<GribTile>>) {
    drop(Box::from_raw(ptr));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_new(point: *mut GeoPoint, date_time: InteropDateTimeStruct, download_path: *const std::ffi::c_char) -> *mut Arc<Mutex<GribTile>> {
    let arc_tile = Arc::new(Mutex::new(GribTile::new(
        &*point,
        &struct_to_rust_date_time(date_time).unwrap(),
        &PathBuf::new().join(c_str_to_rust_str(download_path)),
    )));
    return Box::leak(Box::new(arc_tile));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_bottom_lat(ptr: *mut Arc<Mutex<GribTile>>) -> *mut aviation_calc_util::geo::Latitude {
    let mutex_guard = (&*ptr).lock().unwrap();
    return Box::leak(Box::new(mutex_guard.bottom_lat()));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_top_lat(ptr: *mut Arc<Mutex<GribTile>>) -> *mut aviation_calc_util::geo::Latitude {
    let mutex_guard = (&*ptr).lock().unwrap();
    return Box::leak(Box::new(mutex_guard.top_lat()));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_left_lon(ptr: *mut Arc<Mutex<GribTile>>) -> *mut aviation_calc_util::geo::Longitude {
    let mutex_guard = (&*ptr).lock().unwrap();
    return Box::leak(Box::new(mutex_guard.left_lon()));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_right_lon(ptr: *mut Arc<Mutex<GribTile>>) -> *mut aviation_calc_util::geo::Longitude {
    let mutex_guard = (&*ptr).lock().unwrap();
    return Box::leak(Box::new(mutex_guard.right_lon()));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_center_point(ptr: *mut Arc<Mutex<GribTile>>) -> *mut aviation_calc_util::geo::GeoPoint {
    let mutex_guard = (&*ptr).lock().unwrap();
    return Box::leak(Box::new(mutex_guard.center_point()));
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_is_point_in_tile(ptr: *mut Arc<Mutex<GribTile>>, point: *mut aviation_calc_util::geo::GeoPoint) -> bool {
    let mutex_guard = (&*ptr).lock().unwrap();
    return mutex_guard.is_point_in_tile(&*point);
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_forecast_date_utc(ptr: *mut Arc<Mutex<GribTile>>) -> InteropDateTimeStruct {
    let mutex_guard = (&*ptr).lock().unwrap();
    return rust_date_time_to_struct(&mutex_guard.forecast_date_utc());
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_grib_file_name(ptr: *mut Arc<Mutex<GribTile>>) -> *const std::ffi::c_char {
    let mutex_guard = (&*ptr).lock().unwrap();
    return rust_str_to_c_str(mutex_guard.grib_file_name().to_str().unwrap().to_string());
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_download_url(ptr: *mut Arc<Mutex<GribTile>>) -> *const std::ffi::c_char {
    let mutex_guard = (&*ptr).lock().unwrap();
    return rust_str_to_c_str(mutex_guard.download_url());
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_closest_point(ptr: *mut Arc<Mutex<GribTile>>, point: *mut GeoPoint) -> *mut GribDataPoint {
    let mut mutex_guard = (&*ptr).lock().unwrap();
    return match mutex_guard.closest_point(&*point) {
        Some(data_point) => Box::leak(Box::new(data_point)),
        None => std::ptr::null_mut(),
    };
}

#[no_mangle]
pub unsafe extern "C" fn grib_grib_tile_is_valid(ptr: *mut Arc<Mutex<GribTile>>, date_time: InteropDateTimeStruct) -> bool {
    let mutex_guard = (&*ptr).lock().unwrap();
    return mutex_guard.is_valid(&struct_to_rust_date_time(date_time).unwrap());
}
