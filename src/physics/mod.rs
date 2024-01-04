use aviation_calc_util::{units::{Length, Velocity, Acceleration}, physics};

use crate::interop::{InteropDateTimeStruct, struct_to_rust_duration, rust_duration_to_struct};

/// Calculates Final Velocity from Initial Velocity and Time
/// 
/// Kinematics Equation: V<sub>f</sub> = V<sub>i</sub> + a * t
#[no_mangle]
pub unsafe extern fn physics_kinematics_final_velocity(initial_velocity: *mut Velocity, acceleration: *mut Acceleration, time: InteropDateTimeStruct) -> *mut Velocity {
    return Box::leak(Box::new(physics::kinematics_final_velocity(*initial_velocity, *acceleration, struct_to_rust_duration(time))));
}

/// Calculates Final Velocity from Initial Velocity and Time
/// 
/// Kinematics Equation: V<sub>f</sub> = V<sub>i</sub> + a * t
/// 
/// **Note**: All Units must be kept constant!
#[no_mangle]
pub unsafe extern fn physics_kinematics_final_velocity_general(initial_velocity: f64, acceleration: f64, time: f64) -> f64 {
    return physics::kinematics_final_velocity_general(initial_velocity, acceleration, time);
}

/// Calculates Displacement from Initial Velocity, Final Velocity, and Time
/// 
/// Kinematics Equation: Δx = ((V<sub>i</sub> + V<sub>f</sub>)/2) * t
#[no_mangle]
pub unsafe extern fn physics_kinematics_displacement_1(initial_velocity: *mut Velocity, final_velocity: *mut Velocity, time: InteropDateTimeStruct) -> *mut Length {
    return Box::leak(Box::new(physics::kinematics_displacement_1(*initial_velocity, *final_velocity, struct_to_rust_duration(time))));
}


/// Calculates Displacement from Initial Velocity, Final Velocity, and Time
/// 
/// Kinematics Equation: Δx = ((V<sub>i</sub> + V<sub>f</sub>)/2) * t
/// 
/// **Note**: All Units must be kept constant!
#[no_mangle]
pub unsafe extern fn physics_kinematics_displacement_1_general(initial_velocity: f64, final_velocity: f64, time: f64) -> f64 {
    return physics::kinematics_displacement_1_general(initial_velocity, final_velocity, time);
}

/// Calculates Displacement from Initial Velocity, Acceleration, and Time
/// 
/// Kinematics Equation: Δx = V<sub>i</sub> * t + (1/2) * a * t<sup>2</sup>
#[no_mangle]
pub unsafe extern fn physics_kinematics_displacement_2(initial_velocity: *mut Velocity, acceleration: *mut Acceleration, time: InteropDateTimeStruct) -> *mut Length {
    return Box::leak(Box::new(physics::kinematics_displacement_2(*initial_velocity, *acceleration, struct_to_rust_duration(time))));
}


/// Calculates Displacement from Initial Velocity, Acceleration, and Time
/// 
/// Kinematics Equation: Δx = V<sub>i</sub> * t + (1/2) * a * t<sup>2</sup>
/// 
/// **Note**: All Units must be kept constant!
#[no_mangle]
pub unsafe extern fn physics_kinematics_displacement_2_general(initial_velocity: f64, acceleration: f64, time: f64) -> f64 {
    return physics::kinematics_displacement_2_general(initial_velocity, acceleration, time);
}

/// Calculates Time from Displacement, Initial Velocity, and Final Velocity.
/// 
/// Kinematics Equation: t = 2 * Δx / (V<sub>i</sub> + V<sub>f</sub>)
#[no_mangle]
pub unsafe extern fn physics_kinematics_time_1(displacement: *mut Length, initial_velocity: *mut Velocity, final_velocity: *mut Velocity) -> InteropDateTimeStruct {
    return rust_duration_to_struct(physics::kinematics_time_1(*displacement, *initial_velocity, *final_velocity));
}

/// Calculates Time from Displacement, Initial Velocity, and Final Velocity.
/// 
/// Kinematics Equation: t = 2 * Δx / (V<sub>i</sub> + V<sub>f</sub>)
/// 
/// **Note**: All Units must be kept constant!
#[no_mangle]
pub unsafe extern fn physics_kinematics_time_1_general(displacement: f64, initial_velocity: f64, final_velocity: f64) -> f64 {
    return physics::kinematics_time_1_general(displacement, initial_velocity, final_velocity);
}