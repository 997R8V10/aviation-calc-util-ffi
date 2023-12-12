use aviation_calc_util::magnetic::MagneticModelCoefficients;

use crate::interop::{InteropDateStruct, struct_to_rust_naive_date};

crate::ffi_impl_drop_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients);
crate::ffi_impl_clone_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients);
crate::ffi_impl_eq_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients);
crate::ffi_impl_default_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, n, usize);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, n, usize);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, m, usize);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, m, usize);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, g_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, g_nm, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, h_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, h_nm, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, g_dot_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, g_dot_nm, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, h_dot_nm, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticModelCoefficients, magnetic_magnetic_model_coefficients, h_dot_nm, f64);

#[no_mangle]
pub unsafe extern "C" fn magnetic_magnetic_model_coefficients_get_point_on_date(
    ptr: *mut MagneticModelCoefficients,
    model_epoch: f64,
    date: InteropDateStruct,
) -> *mut MagneticModelCoefficients {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.get_point_on_date(model_epoch, &struct_to_rust_naive_date(date).unwrap())));
}