use aviation_calc_util::math;


#[no_mangle]
unsafe extern "C" fn math_factorial(n: i32) -> i64 {
    return math::factorial(n);
}

#[no_mangle]
unsafe extern "C" fn math_factorial_ratio(n: i32, m: i32) -> f64 {
    return math::factorial_ratio(n, m);
}

// Legendre
mod legendre;

// Polynomial
mod polynomial;