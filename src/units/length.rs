use aviation_calc_util::units::length::Length;

use crate::ffi_impl_all_for_number;

ffi_impl_all_for_number!(Length, units_length);

#[no_mangle]
unsafe extern "C" fn units_length_from_feet(val: f64) -> *mut Length {
    return Box::leak(Box::new(Length::from_feet(val)));
}

#[no_mangle]
unsafe extern "C" fn units_length_from_nautical_miles(val: f64) -> *mut Length {
    return Box::leak(Box::new(Length::from_nautical_miles(val)));
}

#[no_mangle]
unsafe extern "C" fn units_length_from_statute_miles(val: f64) -> *mut Length {
    return Box::leak(Box::new(Length::from_statute_miles(val)));
}

#[no_mangle]
unsafe extern "C" fn units_length_as_meters(ptr: *mut Length) -> f64 {
    return (&*ptr).as_meters();
}

#[no_mangle]
unsafe extern "C" fn units_length_as_feet(ptr: *mut Length) -> f64 {
    return (&*ptr).as_feet();
}

#[no_mangle]
unsafe extern "C" fn units_length_as_nautical_miles(ptr: *mut Length) -> f64 {
    return (&*ptr).as_nautical_miles();
}

#[no_mangle]
unsafe extern "C" fn units_length_as_statute_miles(ptr: *mut Length) -> f64 {
    return (&*ptr).as_statute_miles();
}