use aviation_calc_util::{geo::{GeoTileBounds, Latitude, Longitude, GeoTile, GeoPoint}, units::Angle};


crate::ffi_impl_drop_for_struct!(GeoTileBounds, geo_geo_tile_bounds);
crate::ffi_impl_clone_for_struct!(GeoTileBounds, geo_geo_tile_bounds);
crate::ffi_impl_eq_for_struct!(GeoTileBounds, geo_geo_tile_bounds);
crate::ffi_impl_default_for_struct!(GeoTileBounds, geo_geo_tile_bounds);
crate::ffi_impl_geotile_for_struct!(GeoTileBounds, geo_geo_tile_bounds);

#[no_mangle]
pub unsafe extern "C" fn geo_geo_tile_bounds_new(bot_lat: *mut Latitude, top_lat: *mut Latitude, left_lon: *mut Longitude, right_lon: *mut Longitude) -> *mut GeoTileBounds {
    return Box::leak(Box::new(GeoTileBounds::new(*bot_lat, *top_lat, *left_lon, *right_lon)));
}

#[no_mangle]
pub unsafe extern "C" fn geo_geo_tile_bounds_new_from_point(point: *mut GeoPoint, resolution: *mut Angle) -> *mut GeoTileBounds {
    return Box::leak(Box::new(GeoTileBounds::new_from_point(&*point, *resolution)));
}