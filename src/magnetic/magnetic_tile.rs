use std::sync::Arc;

use aviation_calc_util::{
    geo::{geo_point::GeoPoint, bearing::{self, Bearing}},
    magnetic::{
        magnetic_model::MagneticModel,
        magnetic_tile::{MagneticTile, MagneticTileManager, MAG_TILE_RES},
    },
    units::angle::Angle,
};

use crate::interop::{struct_to_rust_naive_date, InteropDateStruct};

crate::ffi_gen_get_struct_const!(MAG_TILE_RES, Angle, magnetic_magnetic_tile);

// MagneticTileManager
crate::ffi_impl_drop_for_struct!(MagneticTileManager, magnetic_magnetic_tile_magnetictilemanager);

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_tile_magnetictilemanager_new(model: *mut MagneticModel) -> *mut MagneticTileManager {
    let model_owned = Box::from_raw(model);
    return Box::leak(Box::new(MagneticTileManager::new(*model_owned)));
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_tile_magnetictilemanager_find_or_create_tile(ptr: *mut MagneticTileManager, point_ptr: *mut GeoPoint, date: InteropDateStruct) -> *mut Arc<MagneticTile> {
    return Box::leak(Box::new((&*ptr).find_or_create_tile(&*point_ptr, &struct_to_rust_naive_date(date).unwrap())));
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_tile_magnetictilemanager_true_to_magnetic(ptr: *mut MagneticTileManager, point_ptr: *mut GeoPoint, date: InteropDateStruct, true_bearing: *mut Bearing) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).true_to_magnetic(&*point_ptr, &struct_to_rust_naive_date(date).unwrap(), *true_bearing)));
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_tile_magnetictilemanager_magnetic_to_true(ptr: *mut MagneticTileManager, point_ptr: *mut GeoPoint, date: InteropDateStruct, magnetic_bearing: *mut Bearing) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).magnetic_to_true(&*point_ptr, &struct_to_rust_naive_date(date).unwrap(), *magnetic_bearing)));
}
