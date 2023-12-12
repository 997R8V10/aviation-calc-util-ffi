#[macro_export]
macro_rules! ffi_impl_geotile_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _bottom_lat >] (ptr: *mut $obj_type) -> *mut aviation_calc_util::geo::Latitude {
                return Box::leak(Box::new((&*ptr).bottom_lat()));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _top_lat >] (ptr: *mut $obj_type) -> *mut aviation_calc_util::geo::Latitude {
                return Box::leak(Box::new((&*ptr).top_lat()));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _left_lon >] (ptr: *mut $obj_type) -> *mut aviation_calc_util::geo::Longitude {
                return Box::leak(Box::new((&*ptr).left_lon()));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _right_lon >] (ptr: *mut $obj_type) -> *mut aviation_calc_util::geo::Longitude {
                return Box::leak(Box::new((&*ptr).right_lon()));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _center_point >] (ptr: *mut $obj_type) -> *mut aviation_calc_util::geo::GeoPoint {
                return Box::leak(Box::new((&*ptr).center_point()));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _is_point_in_tile >] (ptr: *mut $obj_type, point: *mut aviation_calc_util::geo::GeoPoint) -> bool {
                return (&*ptr).is_point_in_tile(&*point);
            }
        }
    }
}