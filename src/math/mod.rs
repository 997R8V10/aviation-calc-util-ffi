use aviation_calc_util::math::{self, Polynomial};


#[no_mangle]
pub unsafe extern "C" fn math_factorial(n: i32) -> i64 {
    return math::factorial(n);
}

#[no_mangle]
pub unsafe extern "C" fn math_factorial_ratio(n: i32, m: i32) -> f64 {
    return math::factorial_ratio(n, m);
}

#[no_mangle]
pub unsafe extern "C" fn math_interpolate_numbers(start: f64, end: f64, multiplier: f64) -> f64 {
    return math::interpolate_numbers(start, end, multiplier);
}

#[no_mangle]
pub unsafe extern "C" fn math_create_line_equation(x_1: f64, y_1: f64, x_2: f64, y_2: f64) -> *mut Polynomial {
    return match math::create_line_equation(x_1, y_1, x_2, y_2) {
        Some(polynomial) => Box::leak(Box::new(polynomial)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn math_find_2_lines_intersection(m_1: f64, b_1: f64, m_2: f64, b_2: f64, x_1: *mut f64, x_2: *mut f64) {
    (*x_1, *x_2) = math::find_2_lines_intersection(m_1, b_1, m_2, b_2);
}

// Legendre
pub mod legendre;

// Polynomial
pub mod polynomial;