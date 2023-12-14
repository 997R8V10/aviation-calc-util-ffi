use crate::{interop::InteropArrStruct, math::polynomial::{math_polynomial_new, math_polynomial_coefficients, math_polynomial_drop}};

#[test]
fn test_polynomial_1() {
    // Create coefficient array
    unsafe {
        let polynomial_ptr = math_polynomial_new([1.0, 0.0, 1.0].as_mut_ptr(), 3 as usize);

        let ret_coeffs = math_polynomial_coefficients(polynomial_ptr);

        let vec = Vec::from_raw_parts(ret_coeffs.ptr, ret_coeffs.length, ret_coeffs.capacity);

        println!("{:?}", vec);

        math_polynomial_drop(polynomial_ptr);
    }
}
