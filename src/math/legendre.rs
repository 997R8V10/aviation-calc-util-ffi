use aviation_calc_util::math::{legendre::{LegendreManager, self}, Polynomial};

crate::ffi_impl_drop_for_struct!(LegendreManager, math_legendre_legendremanager);

#[no_mangle]
pub unsafe extern "C" fn math_legendre_legendremanager_new() -> *mut LegendreManager {
    return Box::leak(Box::new(LegendreManager::new()));
}

#[no_mangle]
pub unsafe extern "C" fn math_legendre_legendremanager_legendre_function(ptr: *mut LegendreManager, n: i32, m: i32, x: f64) -> f64 {
    let obj = &*ptr;

    return obj.legendre_function(n, m, x);
}

#[no_mangle]
pub unsafe extern "C" fn math_legendre_legendre_polynomial(n: i32) -> *mut Polynomial {    
    return Box::leak(Box::new(legendre::legendre_polynomial(n)));
}