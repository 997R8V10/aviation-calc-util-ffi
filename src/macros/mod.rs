use paste::paste;

#[macro_export]
macro_rules! ffi_impl_general_for_number {
    ($obj_type: ident, $method_prefix: ident) => {
        paste::item! {
            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _new >] (val: f64) -> *mut $obj_type {
                return Box::leak(Box::new($obj_type::new(val)));
            }

            #[no_mangle]
            unsafe extern "C" fn [< $method_prefix _drop >] (ptr: *mut $obj_type) {
                drop(Box::from_raw(ptr));
            }
        }
    };
}

#[macro_export]
macro_rules! ffi_impl_display_for_number {
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
        use std::ops::Add;
        use std::ops::AddAssign;
        use std::ops::Div;
        use std::ops::DivAssign;
        use std::ops::Mul;
        use std::ops::MulAssign;
        use std::ops::Neg;
        use std::ops::Rem;
        use std::ops::RemAssign;
        use std::ops::Sub;
        use std::ops::SubAssign;

        use crate::ffi_impl_assign_op_for_number;
        use crate::ffi_impl_display_for_number;
        use crate::ffi_impl_general_for_number;
        use crate::ffi_impl_methods_for_number;
        use crate::ffi_impl_one_part_op_for_number;
        use crate::ffi_impl_two_part_op_for_number;

        ffi_impl_general_for_number!($obj_type, $method_prefix);
        ffi_impl_display_for_number!($obj_type, $method_prefix);
        ffi_impl_one_part_op_for_number!($obj_type, $method_prefix, neg);
        ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, add);
        ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, sub);
        ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, mul);
        ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, div);
        ffi_impl_two_part_op_for_number!($obj_type, $method_prefix, rem);
        ffi_impl_assign_op_for_number!($obj_type, $method_prefix, add_assign);
        ffi_impl_assign_op_for_number!($obj_type, $method_prefix, sub_assign);
        ffi_impl_assign_op_for_number!($obj_type, $method_prefix, mul_assign);
        ffi_impl_assign_op_for_number!($obj_type, $method_prefix, div_assign);
        ffi_impl_assign_op_for_number!($obj_type, $method_prefix, rem_assign);
        ffi_impl_methods_for_number!($obj_type, $method_prefix);
    };
}
