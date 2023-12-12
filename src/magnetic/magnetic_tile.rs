use aviation_calc_util::{
    geo::{Bearing, GeoPoint, GeoTile, GeoTileBounds},
    magnetic::{MagneticModel, MagneticTile, MagneticTileManager, MagneticField},
    units::Angle,
};

use crate::interop::{struct_to_rust_naive_date, InteropDateStruct, rust_naive_date_to_struct};

// MagneticTileManager
crate::ffi_impl_drop_for_struct!(MagneticTileManager, magnetic_magnetic_tile_manager);

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_manager_new(model: *mut MagneticModel) -> *mut MagneticTileManager {
    let model_owned = Box::from_raw(model);
    return Box::leak(Box::new(MagneticTileManager::new(*model_owned)));
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_manager_find_or_create_tile(ptr: *mut MagneticTileManager, point_ptr: *mut GeoPoint, date: InteropDateStruct) -> *mut MagneticTile {
    let arc_tile = (&*ptr).find_or_create_tile(&*point_ptr, &struct_to_rust_naive_date(date).unwrap());
    let new_tile = (*arc_tile).clone();
    return Box::leak(Box::new(new_tile));
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_manager_true_to_magnetic(ptr: *mut MagneticTileManager, point_ptr: *mut GeoPoint, date: InteropDateStruct, true_bearing: *mut Bearing) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).true_to_magnetic(&*point_ptr, &struct_to_rust_naive_date(date).unwrap(), *true_bearing)));
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_manager_magnetic_to_true(ptr: *mut MagneticTileManager, point_ptr: *mut GeoPoint, date: InteropDateStruct, magnetic_bearing: *mut Bearing) -> *mut Bearing {
    return Box::leak(Box::new((&*ptr).magnetic_to_true(&*point_ptr, &struct_to_rust_naive_date(date).unwrap(), *magnetic_bearing)));
}

// MagneticTile
crate::ffi_gen_get_struct_const_for_struct!(MagneticTile, MAG_TILE_RES, Angle, magnetic_magnetic_tile);
crate::ffi_impl_drop_for_struct!(MagneticTile, magnetic_magnetic_tile);
crate::ffi_impl_clone_for_struct!(MagneticTile, magnetic_magnetic_tile);
crate::ffi_impl_default_for_struct!(MagneticTile, magnetic_magnetic_tile);
crate::ffi_impl_eq_for_struct!(MagneticTile, magnetic_magnetic_tile);
crate::ffi_impl_geotile_for_struct!(MagneticTile, magnetic_magnetic_tile);
crate::ffi_gen_get_struct_var_for_struct!(MagneticTile, magnetic_magnetic_tile, bounds, GeoTileBounds);
crate::ffi_gen_get_struct_var_for_struct!(MagneticTile, magnetic_magnetic_tile, field, MagneticField);
crate::ffi_gen_set_struct_var_for_struct!(MagneticTile, magnetic_magnetic_tile, bounds, GeoTileBounds);
crate::ffi_gen_set_struct_var_for_struct!(MagneticTile, magnetic_magnetic_tile, field, MagneticField);

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_date(ptr: *mut MagneticTile) -> InteropDateStruct {
    return rust_naive_date_to_struct(&(&*ptr).date);
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_set_date(ptr: *mut MagneticTile, new_release_date: InteropDateStruct) {
    (*ptr).date = struct_to_rust_naive_date(new_release_date).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_new(point: *mut GeoPoint, date: InteropDateStruct, model: *mut MagneticModel) -> *mut MagneticTile {
    return Box::leak(Box::new(MagneticTile::new(&*point, &struct_to_rust_naive_date(date).unwrap(), &*model)));
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_tile_is_valid(ptr: *mut MagneticTile, point: *mut GeoPoint, date: InteropDateStruct) -> bool {
    return (&*ptr).is_valid(&*point, &struct_to_rust_naive_date(date).unwrap());
}