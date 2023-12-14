use std::ptr;

use aviation_calc_util::{magnetic::{EARTH_WGS84_SEMI_MAJOR_AXIS, EARTH_WGS84_RECIPROCAL_FLATTENING, WMM_EXPANSION, GEOMAGNETIC_REFERENCE_RADIUS, MagneticModelCoefficients, MagneticModel, MagneticField}, geo::GeoPoint};

use crate::interop::{c_str_to_rust_str, rust_error_to_cstr, rust_naive_date_to_struct, rust_str_to_c_str, struct_to_rust_naive_date, InteropDateStruct, InteropArrStruct, c_arr_to_rust_vec};

crate::ffi_gen_get_primitive_const!(EARTH_WGS84_SEMI_MAJOR_AXIS, f64, magnetic_magnetic_model);
crate::ffi_gen_get_primitive_const!(EARTH_WGS84_RECIPROCAL_FLATTENING, f64, magnetic_magnetic_model);
crate::ffi_gen_get_primitive_const!(WMM_EXPANSION, usize, magnetic_magnetic_model);
crate::ffi_gen_get_primitive_const!(GEOMAGNETIC_REFERENCE_RADIUS, f64, magnetic_magnetic_model);

// Magnetic Model
crate::ffi_impl_drop_for_struct!(MagneticModel, magnetic_magnetic_model);

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_new(
    epoch: f64,
    name: *const std::ffi::c_char,
    release_date: InteropDateStruct,
    coeffs: *mut *mut MagneticModelCoefficients,
    coeffs_size: usize
) -> *mut MagneticModel {
    let v = c_arr_to_rust_vec(InteropArrStruct {
        ptr: coeffs,
        length: coeffs_size as usize,
        capacity: coeffs_size as usize
    });
    let mut new_v = Vec::new();
    for ptr in v.iter() {
        new_v.push(**ptr);
    }

    return Box::leak(Box::new(MagneticModel::new(epoch, &c_str_to_rust_str(name), &struct_to_rust_naive_date(release_date).unwrap(), &new_v)));
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_from_file(filename: *const std::ffi::c_char, error_string: *mut *const std::ffi::c_char) -> *mut MagneticModel {
    let result = MagneticModel::from_file(&c_str_to_rust_str(filename));

    match result {
        Ok(mag_model) => {
            *error_string = ptr::null();
            return Box::leak(Box::new(mag_model));
        }
        Err(err) => {
            *error_string = rust_error_to_cstr(&err);
            return ptr::null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_coeffs(ptr: *mut MagneticModel, n: usize, m: usize) -> *mut MagneticModelCoefficients {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.coeffs(n, m)));
}

crate::ffi_gen_get_primitive_var_for_struct!(MagneticModel, magnetic_magnetic_model, epoch, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModel, magnetic_magnetic_model, epoch, f64);

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_name(ptr: *mut MagneticModel) -> *const std::ffi::c_char {
    return rust_str_to_c_str((&*ptr).name.clone());
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_set_name(ptr: *mut MagneticModel, new_name: *const std::ffi::c_char) {
    (*ptr).name = c_str_to_rust_str(new_name);
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_release_date(ptr: *mut MagneticModel) -> InteropDateStruct {
    return rust_naive_date_to_struct(&(&*ptr).release_date);
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_set_release_date(ptr: *mut MagneticModel, new_release_date: InteropDateStruct) {
    (*ptr).release_date = struct_to_rust_naive_date(new_release_date).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_calculate_field(ptr: *mut MagneticModel, point_ptr: *mut GeoPoint, date: InteropDateStruct) -> *mut MagneticField {
    let obj = &*ptr;
    let point = &*point_ptr;
    return Box::leak(Box::new(obj.calculate_field(point, &struct_to_rust_naive_date(date).unwrap())));
}
