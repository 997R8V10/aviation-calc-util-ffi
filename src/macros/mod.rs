mod struct_as_object;
mod number;

#[macro_export]
macro_rules! ffi_gen_get_primitive_const {
    ($const_name: ident, $const_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _get_const_ $const_name >] () -> $const_type {
                return $const_name;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_get_struct_const {
    ($const_name: ident, $const_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _get_const_ $const_name >] () -> *mut $const_type {
                return Box::leak(Box::new($const_name));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_unit_conv_func {
    ($method_name: ident, $val_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _static_ $method_name >] (val: $val_type) -> $val_type {
                return $method_name(val);
            }
        }
    };
}