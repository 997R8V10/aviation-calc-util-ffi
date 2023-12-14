#[macro_export]
macro_rules! ffi_gen_as_method_for_number {
    ($obj_type: ident, $method_prefix: ident, $get_method_name: ident, $ret_type: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $get_method_name >] (ptr: *mut $obj_type) -> $ret_type {
                return (&*ptr).$get_method_name();
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_from_method_for_number {
    ($obj_type: ident, $method_prefix: ident, $create_method_name: ident, $input_type: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $create_method_name >] (val: $input_type) -> *mut $obj_type {
                return Box::leak(Box::new($obj_type::$create_method_name(val)));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_impl_one_part_op_for_number {
    ($obj_type: ident, $method_prefix: ident, $method: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $method >] (ptr: *mut $obj_type) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.$method()));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_two_part_op_for_number {
    ($obj_type: ident, $method_prefix: ident, $method: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $method >] (ptr: *mut $obj_type, rhs_ptr: *mut $obj_type) -> *mut $obj_type {
                let left_obj = &*ptr;
                let right_obj = &*rhs_ptr;
                return Box::leak(Box::new(left_obj.$method(*right_obj)));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _ $method _f64 >] (ptr: *mut $obj_type, rhs: f64) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.$method(rhs)));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_unit_for_number {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _new >] (val: f64) -> *mut $obj_type {
                return Box::leak(Box::new($obj_type::new(val)));
            }

            #[no_mangle]
            pub unsafe extern "C" fn [< $method_prefix _value >] (ptr: *mut $obj_type) -> f64 {
                let left_obj = &*ptr;
                return left_obj.value();
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_all_for_number {
    ($obj_type: ident, $method_prefix: ident) => {
        crate::ffi_impl_eq_for_struct!($obj_type, $method_prefix);
        crate::ffi_impl_ord_for_struct!($obj_type, $method_prefix);
        crate::ffi_impl_drop_for_struct!($obj_type, $method_prefix);
        crate::ffi_impl_display_for_struct!($obj_type, $method_prefix);
        crate::ffi_impl_clone_for_struct!($obj_type, $method_prefix);
        crate::ffi_impl_one_part_op_for_number!($obj_type, $method_prefix, neg);
        crate::ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, add);
        crate::ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, sub);
        crate::ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, mul);
        crate::ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, div);
        crate::ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, rem);
        crate::ffi_impl_unit_for_number!($obj_type, $method_prefix);
        crate::ffi_impl_default_for_struct!($obj_type, $method_prefix);
    };
}
