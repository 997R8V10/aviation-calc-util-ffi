use std::{path::PathBuf, sync::{Arc, Mutex}};

use aviation_calc_util::{atmos::grib::{GribTileManager, GribTile}, geo::GeoPoint};

use crate::interop::{c_str_to_rust_str, struct_to_rust_date_time, InteropDateTimeStruct};


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