use aviation_calc_util::{
    aviation::{
        calculate_arc_tangent_distance, calculate_bank_angle, calculate_constant_radius_turn, calculate_direct_bearing_after_turn, calculate_max_bank_angle, calculate_max_ground_speed_for_turn,
        calculate_radius_of_turn, find_intersection_to_course, get_headwind_component, calculate_chord_for_turn, calculate_linear_course_intercept, calculate_arc_course_intercept, calculate_turn_lead_distance, calculate_flight_path_angle, calculate_vertical_speed,
    },
    geo::{Bearing, GeoPoint},
    units::{Angle, AngularVelocity, Length, Velocity},
};

#[repr(C, packed)]
pub struct InteropCourseInterceptInfo {
    pub required_course: *mut Bearing,
    pub along_track_distance: *mut Length,
    pub cross_track_error: *mut Length,
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_max_bank_angle(ground_speed: *mut Velocity, bank_limit: *mut Angle, turn_rate: *mut AngularVelocity) -> *mut Angle {
    return Box::leak(Box::new(calculate_max_bank_angle(*ground_speed, *bank_limit, *turn_rate)));
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_radius_of_turn(ground_speed: *mut Velocity, bank_angle: *mut Angle) -> *mut Length {
    return Box::leak(Box::new(calculate_radius_of_turn(*ground_speed, *bank_angle)));
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_bank_angle(radius_of_turn: *mut Length, ground_speed: *mut Velocity) -> *mut Angle {
    return Box::leak(Box::new(calculate_bank_angle(*radius_of_turn, *ground_speed)));
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_constant_radius_turn(
    start_bearing: *mut Bearing,
    turn_amount: *mut Angle,
    wind_bearing: *mut Bearing,
    wind_speed: *mut Velocity,
    true_airspeed: *mut Velocity,
    bank_limit: *mut Angle,
    turn_rate: *mut AngularVelocity,
) -> *mut Length {
    return Box::leak(Box::new(calculate_constant_radius_turn(
        *start_bearing,
        *turn_amount,
        *wind_bearing,
        *wind_speed,
        *true_airspeed,
        *bank_limit,
        *turn_rate,
    )));
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_max_ground_speed_for_turn(
    start_bearing: *mut Bearing,
    turn_amount: *mut Angle,
    wind_bearing: *mut Bearing,
    wind_speed: *mut Velocity,
    true_airspeed: *mut Velocity,
) -> *mut Velocity {
    return Box::leak(Box::new(calculate_max_ground_speed_for_turn(*start_bearing, *turn_amount, *wind_bearing, *wind_speed, *true_airspeed)));
}

#[no_mangle]
pub unsafe extern "C" fn aviation_get_headwind_component(wind_bearing: *mut Bearing, wind_speed: *mut Velocity, bearing: *mut Bearing) -> *mut Velocity {
    return Box::leak(Box::new(get_headwind_component(*wind_bearing, *wind_speed, *bearing)));
}

#[repr(C, packed)]
pub struct InteropChordLine {
    pub bearing: *mut Bearing,
    pub distance: *mut Length,
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_chord_for_turn(start_bearing: *mut Bearing, turn_amount: *mut Angle, radius_of_turn: *mut Length) -> InteropChordLine {
    let (bearing, distance) = calculate_chord_for_turn(*start_bearing, *turn_amount, *radius_of_turn);
    return InteropChordLine {
        bearing: Box::leak(Box::new(bearing)),
        distance: Box::leak(Box::new(distance))
    }
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_direct_bearing_after_turn(start_point: *mut GeoPoint, end_point: *mut GeoPoint, radius_of_turn: *mut Length, start_bearing: *mut Bearing) -> *mut Bearing {
    return match calculate_direct_bearing_after_turn(&*start_point, &*end_point, *radius_of_turn, *start_bearing) {
        Some(bearing) => Box::leak(Box::new(bearing)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_linear_course_intercept(aircraft: *mut GeoPoint, waypoint: *mut GeoPoint, course: *mut Bearing) -> InteropCourseInterceptInfo {
    let intercept_info = calculate_linear_course_intercept(&*aircraft, &*waypoint, *course);
    return InteropCourseInterceptInfo {
        required_course: Box::leak(Box::new(intercept_info.required_course)),
        along_track_distance: Box::leak(Box::new(intercept_info.along_track_distance)),
        cross_track_error: Box::leak(Box::new(intercept_info.cross_track_error))
    };
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_arc_course_intercept(
    aircraft: *mut GeoPoint,
    arc_center: *mut GeoPoint,
    start_radial: *mut Bearing,
    end_radial: *mut Bearing,
    radius: *mut Length,
    is_clockwise: bool,
) -> InteropCourseInterceptInfo {
    let intercept_info = calculate_arc_course_intercept(&*aircraft, &*arc_center, *start_radial, *end_radial, *radius, is_clockwise);
    return InteropCourseInterceptInfo {
        required_course: Box::leak(Box::new(intercept_info.required_course)),
        along_track_distance: Box::leak(Box::new(intercept_info.along_track_distance)),
        cross_track_error: Box::leak(Box::new(intercept_info.cross_track_error))
    };
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_arc_tangent_distance(theta: *mut Angle, r: *mut Length) -> *mut Length {
    return Box::leak(Box::new(calculate_arc_tangent_distance(*theta, *r)));
}

#[repr(C, packed)]
pub struct InteropTurnLeadDistance {
    pub is_null: u8,
    pub turn_lead_dist: *mut Length,
    pub radius_of_turn: *mut Length,
    pub intersection: *mut GeoPoint
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_turn_lead_distance(
    pos: *mut GeoPoint,
    wp: *mut GeoPoint,
    cur_bearing: *mut Bearing,
    true_airspeed: *mut Velocity,
    course: *mut Bearing,
    wind_direction: *mut Bearing,
    wind_speed: *mut Velocity,
    bank_limit: *mut Angle,
    turn_rate: *mut AngularVelocity,
) -> InteropTurnLeadDistance {
    return match calculate_turn_lead_distance(&*pos, &*wp, *cur_bearing, *true_airspeed, *course, *wind_direction, *wind_speed, *bank_limit, *turn_rate) {
        Some((tl_dist, rot, inters)) => {
            InteropTurnLeadDistance {
                is_null: false as u8,
                turn_lead_dist: Box::leak(Box::new(tl_dist)),
                radius_of_turn: Box::leak(Box::new(rot)),
                intersection: Box::leak(Box::new(inters))
            }
        },
        None => {
            InteropTurnLeadDistance {
                is_null: true as u8,
                turn_lead_dist: std::ptr::null_mut(),
                radius_of_turn: std::ptr::null_mut(),
                intersection: std::ptr::null_mut()
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn aviation_find_intersection_to_course(position: *mut GeoPoint, wp: *mut GeoPoint, cur_bearing: *mut Bearing, course: *mut Bearing) -> *mut GeoPoint {
    return match find_intersection_to_course(&*position, &*wp, *cur_bearing, *course) {
        Some(intersection) => Box::leak(Box::new(intersection)),
        None => std::ptr::null_mut()
    };
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_flight_path_angle(ground_speed: *mut Velocity, vertical_speed: *mut Velocity) -> *mut Angle {
    return Box::leak(Box::new(calculate_flight_path_angle(*ground_speed, *vertical_speed)));
}

#[no_mangle]
pub unsafe extern "C" fn aviation_calculate_vertical_speed(ground_speed: *mut Velocity, flight_path_angle: *mut Angle) -> *mut Velocity {
    return Box::leak(Box::new(calculate_vertical_speed(*ground_speed, *flight_path_angle)));
}