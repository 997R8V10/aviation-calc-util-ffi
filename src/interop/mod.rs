use aviation_calc_util::DateTime;
use aviation_calc_util::Datelike;
use aviation_calc_util::NaiveDate;
use aviation_calc_util::Utc;

// C String
#[no_mangle]
pub unsafe extern "C" fn general_free_string(ptr: *mut std::ffi::c_char) {
    drop(std::ffi::CString::from_raw(ptr));
}

pub fn rust_str_to_c_str(str: String) -> *const std::ffi::c_char {
    return std::ffi::CString::new(str).unwrap().into_raw();
}

pub unsafe fn c_str_to_rust_str(ptr: *const std::ffi::c_char) -> String {
    return std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_string();
}

// C Array
#[repr(C, packed)]
pub struct InteropArrStruct<T> {
    pub ptr: *mut T,
    pub length: usize,
    pub capacity: usize
}

#[no_mangle]
pub unsafe extern "C" fn general_free_vec_f64(arr: InteropArrStruct<f64>) {
    drop(Vec::from_raw_parts(arr.ptr, arr.length, arr.capacity));
}

pub fn rust_vec_to_c_arr<T>(vec: Vec<T>) -> InteropArrStruct<T> {
    let capacity = vec.capacity();
    let array = vec.leak();

    return InteropArrStruct {
        ptr: array.as_mut_ptr(),
        length: array.len(),
        capacity
    };
}

pub unsafe fn c_arr_to_rust_vec<T: Copy>(arr: InteropArrStruct<T>) -> Vec<T> {
    let mut v: Vec<T> = Vec::with_capacity(arr.capacity);
    for i in 0..arr.length {
        v.push(*(arr.ptr.wrapping_add(i))); // equivalent of c coefficients[i] or *(coefficients + i * sizeof(double))
    }

    return v;
}

// NaiveDate
#[repr(C, packed)]
pub struct InteropDateStruct {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub fn rust_naive_date_to_struct(date: &NaiveDate) -> InteropDateStruct {
    return InteropDateStruct {
        year: date.year(),
        month: date.month(),
        day: date.day(),
    };
}

pub fn struct_to_rust_naive_date(date_struct: InteropDateStruct) -> Option<NaiveDate> {
    return NaiveDate::from_ymd_opt(date_struct.year, date_struct.month, date_struct.day);
}

// DateTime<Utc>
#[repr(C, packed)]
pub struct InteropDateTimeStruct {
    pub secs: i64,
    pub nsecs: u32,
}

pub fn rust_date_time_to_struct(date_time: &DateTime<Utc>) -> InteropDateTimeStruct {
    return InteropDateTimeStruct {
        secs: date_time.timestamp(),
        nsecs: date_time.timestamp_subsec_nanos(),
    };
}

pub fn struct_to_rust_date_time(date_time_struct: InteropDateTimeStruct) -> Option<DateTime<Utc>> {
    return DateTime::from_timestamp(date_time_struct.secs, date_time_struct.nsecs);
}

// Errors
pub fn rust_error_to_cstr(error: &aviation_calc_util::Error) -> *const std::ffi::c_char {
    return rust_str_to_c_str(error.to_string());
}
