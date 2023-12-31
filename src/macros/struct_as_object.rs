#[macro_export]
macro_rules! ffi_impl_eq_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _eq >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj.eq(right_obj);
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ne >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj.ne(right_obj);
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_clone_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _clone >] (ptr: *mut $obj_type) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.clone()));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_default_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _default >] () -> *mut $obj_type {
                return Box::leak(Box::new($obj_type::default()));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_ord_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _compare >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> i32 {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                let comp_val = left_obj.partial_cmp(right_obj);

                if (comp_val.is_none()) {
                    return 0;
                }

                if (comp_val.unwrap() == std::cmp::Ordering::Less) {
                    return -1;
                }

                if (comp_val.unwrap() == std::cmp::Ordering::Greater) {
                    return 1;
                }

                return 0;
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _lt >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj.lt(right_obj);
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _le >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj.le(right_obj);
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _gt >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj.gt(right_obj);
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ge >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj.ge(right_obj);
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_drop_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _drop >] (ptr: *mut $obj_type) {
                drop(Box::from_raw(ptr));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_get_primitive_var_for_struct {
    ($obj_type: ident, $method_prefix: ident, $var_name: ident, $var_type: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $var_name >] (ptr: *mut $obj_type) -> $var_type {
                return (&*ptr).$var_name;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_set_primitive_var_for_struct {
    ($obj_type: ident, $method_prefix: ident, $var_name: ident, $var_type: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _set_ $var_name >] (ptr: *mut $obj_type, val: $var_type) {
                (*ptr).$var_name = val;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_get_primitive_const_for_struct {
    ($obj_type: ident, $const_name: ident, $const_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _get_const_ $const_name >] () -> $const_type {
                return $obj_type::$const_name;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_get_struct_const_for_struct {
    ($obj_type: ident, $const_name: ident, $const_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _get_const_ $const_name >] () -> *mut $const_type {
                return Box::leak(Box::new($obj_type::$const_name));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_get_struct_var_for_struct {
    ($obj_type: ident, $method_prefix: ident, $var_name: ident, $var_type: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $var_name >] (ptr: *mut $obj_type) -> *mut $var_type {
                return Box::leak(Box::new((&*ptr).$var_name));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_set_struct_var_for_struct {
    ($obj_type: ident, $method_prefix: ident, $var_name: ident, $var_type: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _set_ $var_name >] (ptr: *mut $obj_type, val: *mut $var_type) {
                (*ptr).$var_name = *val;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_impl_display_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _to_string >] (ptr: *mut $obj_type) -> *const std::ffi::c_char {
                return std::ffi::CString::new((&*ptr).to_string()).unwrap().into_raw();
            }
        }
    }
}