use aviation_calc_util::math::Polynomial;

use crate::interop::{InteropArrStruct, rust_vec_to_c_arr, c_arr_to_rust_vec};

crate::ffi_impl_drop_for_struct!(Polynomial, math_polynomial);

#[no_mangle]
pub unsafe extern "C" fn math_polynomial_new(coefficients: InteropArrStruct<f64>) -> *mut Polynomial {
    let v = c_arr_to_rust_vec(coefficients);
    return Box::leak(Box::new(Polynomial::new(&v)));
}

#[no_mangle]
pub unsafe extern "C" fn math_polynomial_evaluate(ptr: *mut Polynomial, x: f64) -> f64 {
    let obj = &*ptr;
    return obj.evaluate(x);
}

#[no_mangle]
pub unsafe extern "C" fn math_polynomial_derivative(ptr: *mut Polynomial, n: i32) -> *mut Polynomial {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.derivative(n)));
}

#[no_mangle]
pub unsafe extern "C" fn math_polynomial_coefficients(ptr: *mut Polynomial) -> InteropArrStruct<f64> {
    let obj = &*ptr;

    return rust_vec_to_c_arr(obj.coefficients());
}