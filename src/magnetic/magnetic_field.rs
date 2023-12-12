use aviation_calc_util::{magnetic::{MagneticFieldElements, MagneticField}, units::Angle, geo::Bearing};

// MagneticFieldElements
crate::ffi_impl_drop_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements);
crate::ffi_impl_clone_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements);
crate::ffi_impl_eq_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements);
crate::ffi_impl_default_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, x, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, x, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, y, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, y, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, z, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, z, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, h, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, h, f64);
crate::ffi_gen_get_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, f, f64);
crate::ffi_gen_set_primitive_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, f, f64);
crate::ffi_gen_get_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, incl, Angle);
crate::ffi_gen_set_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, incl, Angle);
crate::ffi_gen_get_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, decl, Angle);
crate::ffi_gen_set_struct_var_for_struct!(MagneticFieldElements, magnetic_magnetic_field_elements, decl, Angle);

// MagneticField
crate::ffi_impl_drop_for_struct!(MagneticField, magnetic_magnetic_field);
crate::ffi_impl_clone_for_struct!(MagneticField, magnetic_magnetic_field);
crate::ffi_impl_eq_for_struct!(MagneticField, magnetic_magnetic_field);
crate::ffi_impl_default_for_struct!(MagneticField, magnetic_magnetic_field);
crate::ffi_gen_get_struct_var_for_struct!(MagneticField, magnetic_magnetic_field, field_elements, MagneticFieldElements);
crate::ffi_gen_set_struct_var_for_struct!(MagneticField, magnetic_magnetic_field, field_elements, MagneticFieldElements);
crate::ffi_gen_get_struct_var_for_struct!(MagneticField, magnetic_magnetic_field, sec_elements, MagneticFieldElements);
crate::ffi_gen_set_struct_var_for_struct!(MagneticField, magnetic_magnetic_field, sec_elements, MagneticFieldElements);

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_field_true_to_magnetic(ptr: *mut MagneticField, true_bearing: *mut Bearing) -> *mut Bearing {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.true_to_magnetic(*true_bearing)));
}

#[no_mangle]
unsafe extern "C" fn magnetic_magnetic_field_magnetic_to_true(ptr: *mut MagneticField, magnetic_bearing: *mut Bearing) -> *mut Bearing {
    let obj = &*ptr;
    return Box::leak(Box::new(obj.magnetic_to_true(*magnetic_bearing)));
}