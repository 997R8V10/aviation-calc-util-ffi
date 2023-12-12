use std::ptr;

use aviation_calc_util::{
    geo::{bearing::Bearing, geo_point::GeoPoint},
    magnetic::magnetic_model::{
        MagneticField, MagneticFieldElements, MagneticModel, MagneticModelCoefficients, EARTH_WGS84_RECIPROCAL_FLATTENING, EARTH_WGS84_SEMI_MAJOR_AXIS, GEOMAGNETIC_REFERENCE_RADIUS, WMM_EXPANSION,
    },
    units::angle::Angle,
};

use crate::interop::{c_str_to_rust_str, rust_error_to_cstr, rust_naive_date_to_struct, rust_str_to_c_str, struct_to_rust_naive_date, InteropDateStruct, InteropArrStruct, c_arr_to_rust_vec};

crate::ffi_gen_get_primitive_const!(EARTH_WGS84_SEMI_MAJOR_AXIS, f64, magnetic_magnetic_model);
crate::ffi_gen_get_primitive_const!(EARTH_WGS84_RECIPROCAL_FLATTENING, f64, magnetic_magnetic_model);
crate::ffi_gen_get_primitive_const!(WMM_EXPANSION, usize, magnetic_magnetic_model);
crate::ffi_gen_get_primitive_const!(GEOMAGNETIC_REFERENCE_RADIUS, f64, magnetic_magnetic_model);

// Magnetic Model
crate::ffi_impl_drop_for_struct!(MagneticModel, magnetic_magnetic_model_magneticmodel);

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_new(
    epoch: f64,
    name: *const std::ffi::c_char,
    release_date: InteropDateStruct,
    coeffs: InteropArrStruct<*mut MagneticModelCoefficients>,
) -> *mut MagneticModel {
    let v = c_arr_to_rust_vec(coeffs);
    let mut new_v = Vec::new();
    for ptr in v.iter() {
        new_v.push(**ptr);
    }

    return Box::leak(Box::new(MagneticModel::new(epoch, &c_str_to_rust_str(name), &struct_to_rust_naive_date(release_date).unwrap(), &new_v)));
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_from_file(filename: *const std::ffi::c_char, error_string: *mut *const std::ffi::c_char) -> *mut MagneticModel {
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
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_coeffs(ptr: *mut MagneticModel, n: usize, m: usize) -> *mut MagneticModelCoefficients {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.coeffs(n, m)));
}

crate::ffi_gen_get_primitive_var_for_struct!(MagneticModel, magnetic_magnetic_model_magneticmodel, epoch, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModel, magnetic_magnetic_model_magneticmodel, epoch, f64);

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_name(ptr: *mut MagneticModel) -> *const std::ffi::c_char {
    return rust_str_to_c_str((&*ptr).name.clone());
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_set_name(ptr: *mut MagneticModel, new_name: *const std::ffi::c_char) {
    (*ptr).name = c_str_to_rust_str(new_name);
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_release_date(ptr: *mut MagneticModel) -> InteropDateStruct {
    return rust_naive_date_to_struct(&(&*ptr).release_date);
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_set_release_date(ptr: *mut MagneticModel, new_release_date: InteropDateStruct) {
    (*ptr).release_date = struct_to_rust_naive_date(new_release_date).unwrap();
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodel_calculate_field(ptr: *mut MagneticModel, point_ptr: *mut GeoPoint, date: InteropDateStruct) -> *mut MagneticField {
    let obj = &*ptr;
    let point = &*point_ptr;
    return Box::leak(Box::new(obj.calculate_field(point, &struct_to_rust_naive_date(date).unwrap())));
}

// MagneticModelCoefficients
crate::ffi_impl_drop_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients);
crate::ffi_impl_clone_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients);
crate::ffi_impl_eq_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients);
crate::ffi_impl_default_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, n, usize);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, n, usize);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, m, usize);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, m, usize);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, g_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, g_nm, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, h_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, h_nm, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, g_dot_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, g_dot_nm, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, h_dot_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_magneticmodelcoefficients, h_dot_nm, f64);

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticmodelcoefficients_get_point_on_date(
    ptr: *mut MagneticModelCoefficients,
    model_epoch: f64,
    date: InteropDateStruct,
) -> *mut MagneticModelCoefficients {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.get_point_on_date(model_epoch, &struct_to_rust_naive_date(date).unwrap())));
}

// MagneticFieldElements
crate::ffi_impl_drop_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements);
crate::ffi_impl_clone_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements);
crate::ffi_impl_eq_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements);
crate::ffi_impl_default_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, x, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, x, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, y, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, y, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, z, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, z, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, h, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, h, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, f, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, f, f64);
crate::ffi_gen_get_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, incl, Angle);
crate::ffi_gen_set_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, incl, Angle);
crate::ffi_gen_get_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, decl, Angle);
crate::ffi_gen_set_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_model_magneticfieldelements, decl, Angle);

// MagneticField
crate::ffi_impl_drop_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield);
crate::ffi_impl_clone_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield);
crate::ffi_impl_eq_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield);
crate::ffi_impl_default_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield);
crate::ffi_gen_get_struct_var_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield, field_elements, MagneticFieldElements);
crate::ffi_gen_set_struct_var_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield, field_elements, MagneticFieldElements);
crate::ffi_gen_get_struct_var_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield, sec_elements, MagneticFieldElements);
crate::ffi_gen_set_struct_var_for_struct!(MagneticField, magnetic_magnetic_model_magneticfield, sec_elements, MagneticFieldElements);

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticfield_true_to_magnetic(ptr: *mut MagneticField, true_bearing: *mut Bearing) -> *mut Bearing {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.true_to_magnetic(*true_bearing)));
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_model_magneticfield_magnetic_to_true(ptr: *mut MagneticField, magnetic_bearing: *mut Bearing) -> *mut Bearing {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.magnetic_to_true(*magnetic_bearing)));
}
