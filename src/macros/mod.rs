use paste::paste;

#[macro_export]
macro_rules! ffi_impl_eq_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _equals >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> bool {
                let left_obj = &*ptr;
                let right_obj = &*other_ptr;

                return left_obj == right_obj;
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_clone_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _clone >] (ptr: *mut $obj_type) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.clone()));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_ord_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _compare >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> i32 {
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
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_drop_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _drop >] (ptr: *mut $obj_type) {
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
            unsafe extern "C" fn [< $method_prefix _get_ $var_name >] (ptr: *mut $obj_type) -> $var_type {
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
            unsafe extern "C" fn [< $method_prefix _set_ $var_name >] (ptr: *mut $obj_type, val: $var_type) {
                (&*ptr).$var_name = val;
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_gen_as_method_for_number {
    ($obj_type: ident, $method_prefix: ident, $get_method_name: ident, $ret_type: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $get_method_name >] (ptr: *mut $obj_type) -> $ret_type {
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
            unsafe extern "C" fn [< $method_prefix _ $create_method_name >] (val: $input_type) -> *mut $obj_type {
                return Box::leak(Box::new($obj_type::$create_method_name(val)));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_impl_display_for_struct {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _to_string >] (ptr: *mut $obj_type) -> *const std::ffi::c_char {
                return std::ffi::CString::new((&*ptr).to_string()).unwrap().into_raw();
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_one_part_op_for_number {
    ($obj_type: ident, $method_prefix: ident, $method: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method >] (ptr: *mut $obj_type) -> *mut $obj_type {
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
            unsafe extern "C" fn [< $method_prefix _ $method >] (ptr: *mut $obj_type, rhs_ptr: *mut $obj_type) -> *mut $obj_type {
                let left_obj = &*ptr;
                let right_obj = &*rhs_ptr;
                return Box::leak(Box::new(left_obj.$method(*right_obj)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _f64 >] (ptr: *mut $obj_type, rhs: f64) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.$method(rhs)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _f32 >] (ptr: *mut $obj_type, rhs: f32) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.$method(rhs)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _i32 >] (ptr: *mut $obj_type, rhs: i32) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.$method(rhs)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _i64 >] (ptr: *mut $obj_type, rhs: i64) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.$method(rhs)));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_assign_op_for_number {
    ($obj_type: ident, $method_prefix: ident, $method: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method >] (ptr: *mut $obj_type, rhs_ptr: *mut $obj_type) {
                let left_obj = &mut *ptr;
                let right_obj = &*rhs_ptr;
                left_obj.$method(*right_obj);
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _f64 >] (ptr: *mut $obj_type, rhs: f64) {
                let left_obj = &mut *ptr;
                left_obj.$method(rhs);
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _f32 >] (ptr: *mut $obj_type, rhs: f32) {
                let left_obj = &mut *ptr;
                left_obj.$method(rhs);
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _i32 >] (ptr: *mut $obj_type, rhs: i32) {
                let left_obj = &mut *ptr;
                left_obj.$method(rhs);
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _ $method _i64 >] (ptr: *mut $obj_type, rhs: i64) {
                let left_obj = &mut *ptr;
                left_obj.$method(rhs);
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_methods_for_number {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _abs >] (ptr: *mut $obj_type) -> *mut $obj_type {
                let left_obj = &*ptr;
                return Box::leak(Box::new(left_obj.abs()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _powi >] (ptr: *mut $obj_type, n: i32) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).powi(n)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _powf >] (ptr: *mut $obj_type, n: f64) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).powf(n)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _sqrt >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).sqrt()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _sin >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).sin()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _cos >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).cos()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _tan >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).tan()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _sinh >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).sinh()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _cosh >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).cosh()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _tanh >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).tanh()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _asin >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).asin()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _acos >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).acos()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _atan >] (ptr: *mut $obj_type) -> *mut $obj_type {
                return Box::leak(Box::new((&*ptr).atan()));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _atan2 >] (ptr: *mut $obj_type, other_ptr: *mut $obj_type) -> *mut $obj_type {
                let other = &*other_ptr;
                return Box::leak(Box::new((&*ptr).atan2(*other)));
            }
        }
    }
}

#[macro_export]
macro_rules! ffi_impl_all_for_number {
    ($obj_type: ident, $method_prefix: ident) => {

        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _new >] (val: f64) -> *mut $obj_type {
                return Box::leak(Box::new($obj_type::new(val)));
            }
        }

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
        crate::ffi_impl_assign_op_for_number!($obj_type, $method_prefix, add_assign);
        crate::ffi_impl_assign_op_for_number!($obj_type, $method_prefix, sub_assign);
        crate::ffi_impl_assign_op_for_number!($obj_type, $method_prefix, mul_assign);
        crate::ffi_impl_assign_op_for_number!($obj_type, $method_prefix, div_assign);
        crate::ffi_impl_assign_op_for_number!($obj_type, $method_prefix, rem_assign);
        crate::ffi_impl_methods_for_number!($obj_type, $method_prefix);
    };
}
