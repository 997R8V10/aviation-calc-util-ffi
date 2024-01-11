use aviation_calc_util::{atmos::{R_DRY_AIR, SPEC_HEAT_RATIO_AIR, ISA_STD_TEMP, ISA_STD_PRES, ISA_STD_DENS, ISA_STD_LAPSE_RATE, ISA_STRATO_TEMP, ISA_STD_PRES_DROP_PER_ALT, calculate_dry_air_density, calculate_pressure_at_alt, calculate_temp_at_alt, calculate_density_altitude, calculate_impact_pressure_at_cas, calculate_calibrated_airspeed, calculate_mach_number, calculate_eas, calculate_speed_of_sound_dry_air, convert_mach_to_tas, calculate_impact_pressure, convert_tas_to_mach, convert_ias_to_tas, convert_indicated_to_absolute_alt, convert_absolute_to_indicated_alt, convert_indicated_to_pressure_alt, calculate_isa_temp, convert_tas_to_ias}, units::{Temperature, Pressure, Velocity, Length}};

pub mod grib;

crate::ffi_gen_get_primitive_const!(R_DRY_AIR, f64, atmos);
crate::ffi_gen_get_primitive_const!(SPEC_HEAT_RATIO_AIR, f64, atmos);
crate::ffi_gen_get_struct_const!(ISA_STD_TEMP, Temperature, atmos);
crate::ffi_gen_get_struct_const!(ISA_STD_PRES, Pressure, atmos);
crate::ffi_gen_get_primitive_const!(ISA_STD_DENS, f64, atmos);
crate::ffi_gen_get_primitive_const!(ISA_STD_LAPSE_RATE, f64, atmos);
crate::ffi_gen_get_struct_const!(ISA_STRATO_TEMP, Temperature, atmos);
crate::ffi_gen_get_primitive_const!(ISA_STD_PRES_DROP_PER_ALT, f64, atmos);

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_dry_air_density(p: *mut Pressure, t: *mut Temperature) -> f64 {
    return calculate_dry_air_density(*p, *t);
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_pressure_at_alt(h: *mut Length, h_0: *mut Length, p_0:*mut  Pressure, t: *mut Temperature) -> *mut Pressure {
    return Box::leak(Box::new(calculate_pressure_at_alt(*h, *h_0, *p_0, *t)));
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_temp_at_alt(h: *mut Length, h_0: *mut Length, t_0: *mut Temperature) -> *mut Temperature {
    return Box::leak(Box::new(calculate_temp_at_alt(*h, *h_0, *t_0)));
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_density_altitude(p: *mut Pressure, t: *mut Temperature) -> *mut Length {
    return Box::leak(Box::new(calculate_density_altitude(*p, *t))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_impact_pressure_at_cas(cas: *mut Velocity) -> *mut Pressure {
    return Box::leak(Box::new(calculate_impact_pressure_at_cas(*cas))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_calibrated_airspeed(qc: *mut Pressure) -> *mut Velocity {
    return Box::leak(Box::new(calculate_calibrated_airspeed(*qc))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_mach_number(qc: *mut Pressure, p: *mut Pressure) -> f64 {
    return calculate_mach_number(*qc, *p);
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_eas(m: f64, p: *mut Pressure) -> *mut Velocity {
    return Box::leak(Box::new(calculate_eas(m, *p))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_speed_of_sound_dry_air(t: *mut Temperature) -> *mut Velocity {
    return Box::leak(Box::new(calculate_speed_of_sound_dry_air(*t))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_mach_to_tas(m: f64, t: *mut Temperature) -> *mut Velocity {
    return Box::leak(Box::new(convert_mach_to_tas(m, *t))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_impact_pressure(m: f64, p: *mut Pressure) -> *mut Pressure {
    return Box::leak(Box::new(calculate_impact_pressure(m, *p))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_tas_to_mach(tas: *mut Velocity, t: *mut Temperature) -> f64 {
    return convert_tas_to_mach(*tas, *t);
}

#[repr(C)]
pub struct InteropSpeedMachStruct {
    pub speed: *mut Velocity,
    pub mach: f64,
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_ias_to_tas(ias: *mut Velocity, ref_press: *mut Pressure, alt: *mut Length, ref_alt: *mut Length, ref_temp: *mut Temperature) -> InteropSpeedMachStruct {
    let ret_val = convert_ias_to_tas(*ias, *ref_press, *alt, *ref_alt, *ref_temp);
    return InteropSpeedMachStruct {
        speed: Box::leak(Box::new(ret_val.0)),
        mach: ret_val.1
    };
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_tas_to_ias(tas: *mut Velocity, ref_press: *mut Pressure, alt: *mut Length, ref_alt: *mut Length, ref_temp: *mut Temperature) -> InteropSpeedMachStruct {
    let ret_val = convert_tas_to_ias(*tas, *ref_press, *alt, *ref_alt, *ref_temp);
    return InteropSpeedMachStruct {
        speed: Box::leak(Box::new(ret_val.0)),
        mach: ret_val.1
    };
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_indicated_to_absolute_alt(alt_ind: *mut Length, pres_set: *mut Pressure, sfc_pres: *mut Pressure) -> *mut Length {
    return Box::leak(Box::new(convert_indicated_to_absolute_alt(*alt_ind, *pres_set, *sfc_pres))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_absolute_to_indicated_alt(alt_abs: *mut Length, pres_set: *mut Pressure, sfc_pres: *mut Pressure) -> *mut Length {
    return Box::leak(Box::new(convert_absolute_to_indicated_alt(*alt_abs, *pres_set, *sfc_pres))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_convert_indicated_to_pressure_alt(alt_ind: *mut Length, pres_set: *mut Pressure) -> *mut Length {
    return Box::leak(Box::new(convert_indicated_to_pressure_alt(*alt_ind, *pres_set))); 
}

#[no_mangle]
pub unsafe extern "C" fn atmos_calculate_isa_temp(alt_pres: *mut Length) -> *mut Temperature {
    return Box::leak(Box::new(calculate_isa_temp(*alt_pres))); 
}
