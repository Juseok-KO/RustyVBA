use crate::Pointer;

pub mod string;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeCode {
    None = 0,
    I8 = 1,
    I16 = 2,
    I32 = 3,
    I64 = 4,
    F32 = 5,
    F64 = 6,
    BOOL = 7,
    CSTRING = 8,
    ARRAY = 9,
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    I8(i8), 
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    BOOL(bool),
    CSTRING(string::CSTRING),
    Array(Vec<Data>),
    
}


#[derive(Debug, Clone)]
pub struct Data {
    t: TypeCode,
    d: Value
}

impl Data {
    pub fn into_raw_pointer(self) -> *mut Pointer {
        Box::leak(Box::new(self)) as *mut Data as *mut Pointer
    }

    pub fn from_raw_pointer(pointer: *mut Pointer) -> Self {
        let boxed_val = unsafe { Box::from_raw(pointer as *mut Data) };
        Clone::clone(&(*boxed_val))
    }

    pub fn get_type(&self) -> TypeCode {
        self.t
    }

    pub fn get_type_from_ptr(pointer: *mut Pointer) -> TypeCode {
        let refr = unsafe {&*(pointer as *mut Data)};
        refr.get_type()
    }


    pub fn drop(pointer: *mut Pointer) {
        let recovered_val = unsafe { Box::from_raw(pointer as *mut Data )};
        drop(recovered_val);
    }

    pub fn is_none(pointer: *mut Pointer) -> bool {
        let reference = unsafe { &*(pointer as *mut Data) as &Data};
        if let Value::None = reference.d {
            true
        } else {
            false
        }
    }

    pub fn into_vec(self) -> Result<Vec<Data>, Self> {
        let Value::Array(v) = self.d else {
            return Err(self)
        };
        Ok(v)
    }

    pub fn init_array(row: i64, col: i64) -> *mut Pointer {

        let allocated_memory = (0..row).into_iter().map(|_r| {
            (0..col).into_iter().map(|_c| Data {t: TypeCode::None, d: Value::None})
            .collect::<Vec<Data>>()
        }).collect::<Vec<Vec<Data>>>();

        let data = Data::from(allocated_memory);
        data.into_raw_pointer()
    }

    pub fn get_mut_ref_arr_element(pointer: *mut Pointer, row: i64, col: i64, _t: &()) -> Result<&mut Data, String> {
        let mut_ref = unsafe { &mut*(pointer as *mut Data) };

        let Value::Array(arr) = &mut mut_ref.d else {
            return Err(format!("Passed data is not an Array"))
        };

        let Some(r) = arr.get_mut(row as usize) else{
            return Err(format!("Passed Array does not have enough rows"))
        };

        let Value::Array(arr_row) = &mut r.d else {
            return Err(format!("Passed data is not a 2-dimensional Array"))
        };

        let Some(c) = arr_row.get_mut(col as usize) else {
            return Err(format!("Passed Array does not have enough columns"))
        };
        Ok(c)
    }

    pub fn get_ptr_arr_element(pointer: *const Pointer, row: i64, col: i64) -> Result<*const Pointer, String> {
        let t = ();
        let refr = Data::get_ref_arr_element(pointer, row, col, &t)?;
        Ok(refr as *const Data as *const Pointer)
    }

    pub fn get_ref_arr_element(pointer: *const Pointer, row: i64, col: i64, _t: &()) -> Result<&Data, String> {
        let refr = unsafe {&*(pointer as *mut Data) };

        let Value::Array(arr) = & refr.d else {
            return Err(format!("Passed data is not an Array"))
        };

        let Some(r) = arr.get(row as usize) else {
            return Err(format!("Passed Array does not have enough rows"))
        };

        let Value::Array(arr_row) = &r.d else {
            return Err(format!("Passed data is not a 2-dimensional Array"))
        };

        let Some(c) = arr_row.get(col as usize) else {
            return Err(format!("Passed Array does not have enough columns"))
        };

        Ok(c)
    }

    pub fn get_i8(pointer: *const Pointer) -> Result<i8, String>{
        let refr = unsafe { &*(pointer as *const Data)};
        let Value::I8(v) = &refr.d else {
            return Err(format!("Passed value is not I8: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_i16(pointer: *const Pointer) -> Result<i16, String> {
        let refr = unsafe { &*(pointer as *const Data) };
        let Value::I16(v) = &refr.d else {
            return Err(format!("Passed value is not I16: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_i32(pointer: *const Pointer) -> Result<i32, String> {
        let refr = unsafe { &*(pointer as *const Data)};
        let Value::I32(v) = &refr.d else {
            return Err(format!("Passed value is not I32: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_i64(pointer: *const Pointer) -> Result<i64, String> {
        let refr = unsafe { &*(pointer as *const Data)};
        let Value::I64(v) = &refr.d else {
            return Err(format!("Passed value is not I64: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_f32(pointer: *const Pointer) -> Result<f32, String> {
        let refr = unsafe { &*(pointer as *const Data)};
        let Value::F32(v) = &refr.d else {
            return Err(format!("Passed value is not F32: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_f64(pointer: *const Pointer) -> Result<f64, String> {
        let refr = unsafe { &*(pointer as *const Data)};
        let Value::F64(v) = &refr.d else {
            return Err(format!("Passed value is not F64: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_bool(pointer: *const Pointer) -> Result<bool, String> {
        let refr  = unsafe { &*(pointer as *const Data)};
        let Value::BOOL(v) = &refr.d else {
            return Err(format!("Passed value is not BOOL: {:?}", refr.t))
        };
        Ok(*v)
    }

    pub fn get_ptr_str(pointer: *const Pointer) -> Result<*const Pointer, String> {
        let refr = unsafe {&*(pointer as *const Data)};
        let Value::CSTRING(v) = &refr.d else {
            return Err(format!("Passed value is not CSTRING: {:?}", refr.t))
        };

        Ok(v as *const string::CSTRING as *const Pointer)
    }



    pub fn into_i8(pointer: *mut Pointer) -> Result<i8, String> {
        let reference =     unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::I8 => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::I8(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get I8 from {:?}", t))
            }
        }
    }

    pub fn into_i16(pointer: *mut Pointer) -> Result<i16, String> {
        let reference = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::I16 => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::I16(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get I16 from {:?}", t))
            }
        }
    }

    pub fn into_i32(pointer: *mut Pointer) -> Result<i32, String> {
        let reference = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::I32 => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::I32(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get I32 from {:?}", t))
            }
        }
    }

    pub fn into_i64(pointer: *mut Pointer) -> Result<i64, String> {
        let reference = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::I64 => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::I64(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get I64 from {:?}", t))
            }
        }
    }

    pub fn into_f32(pointer: *mut Pointer) -> Result<f32, String> {
        let reference = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::F32 => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::F32(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get F32 from {:?}", t))
            }
        }
    }

    pub fn into_f64(pointer: *mut Pointer) -> Result<f64, String> {
        let reference = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::F64 => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::F64(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get F64 from {:?}", t))
            }
        }
    }

    pub fn into_bool(pointer: *mut Pointer) -> Result<bool, String> {
        let reference  = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::BOOL => {
                let data = Self::from_raw_pointer(pointer);
                if let Value::BOOL(value) = data.d {
                    Ok(value)
                } else {
                    Err(format!("{}", "ss"))
                }
            }
            t @ _ => {
                Err(format!("Failed to get BOOL from {:?}", t))
            }
        }
    }

    pub fn set_i8(pointer: *mut Pointer, row: i64, col: i64, val: i8) -> Result<(), String>{
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::I8;
        mut_ref.d = Value::I8(val);
        Ok(())
    }

    pub fn set_i16(pointer: *mut Pointer, row: i64, col: i64, val: i16) -> Result<(), String> {
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::I16;
        mut_ref.d = Value::I16(val);
        Ok(())
    }

    pub fn set_i32(pointer: *mut Pointer, row: i64, col: i64, val: i32) -> Result<(), String> {
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::I32;
        mut_ref.d = Value::I32(val);
        Ok(())
    }

    pub fn set_i64(pointer: *mut Pointer, row: i64, col: i64, val: i64) -> Result<(), String> {
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::I64;
        mut_ref.d = Value::I64(val);
        Ok(())
    }

    pub fn set_f32(pointer: *mut Pointer, row: i64, col: i64, val: f32) -> Result<(), String> {
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::F32;
        mut_ref.d = Value::F32(val);
        Ok(())
    }

    pub fn set_f64(pointer: *mut Pointer, row: i64, col: i64, val: f64) -> Result<(), String> {
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::F64;
        mut_ref.d = Value::F64(val);
        Ok(())
    }

    pub fn set_bool(pointer: *mut Pointer, row: i64, col: i64, val: bool) -> Result<(), String>{
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::BOOL;
        mut_ref.d = Value::BOOL(val);
        Ok(())
    }

    pub fn set_none(pointer: *mut Pointer, row: i64, col: i64) -> Result<(), String>{
        let t = ();
        let mut_ref = Data::get_mut_ref_arr_element(pointer, row, col, &t)?;
        mut_ref.t = TypeCode::None;
        mut_ref.d = Value::None;
        Ok(())
    }

}

impl From<i8> for Data {
    fn from(value: i8) -> Self {
        Data {
            t: TypeCode::I8,
            d: Value::I8(value)
        }
    }
}

impl From<i16> for Data {
    fn from(value: i16) -> Self {
        Data {
            t: TypeCode::I16,
            d: Value::I16(value)
        }
    }
}

impl From<i32> for Data {
    fn from(value: i32) -> Self {
        Data {
            t: TypeCode::I32,
            d: Value::I32(value)
        }
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data {
            t: TypeCode::I64,
            d: Value::I64(value)
        }
    }
}

impl From<f32> for Data {
    fn from(value: f32) -> Self {
        Data {
            t: TypeCode::F32,
            d: Value::F32(value)
        }
    }
}

impl From<f64> for Data {
    fn from(value: f64) -> Self {
        Data {
            t: TypeCode::F64,
            d: Value::F64(value)
        }
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data {
            t: TypeCode::BOOL,
            d: Value::BOOL(value)
        }
    }
}

impl From<string::CSTRING> for Data {
    fn from(value: string::CSTRING) -> Self {
        Data {
            t: TypeCode::CSTRING,
            d: Value::CSTRING(value)
        }
    }
}

impl<D> From<Vec<D>> for Data 
where Data: From<D>{
    fn from(value: Vec<D>) -> Self {
        Data {
            t: TypeCode::ARRAY,
            d: Value::Array(value.into_iter().map(|d| Data::from(d)).collect::<Vec<Data>>())
        }
    }
}

#[test]
fn vector_test() {

    let vec = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8]
    ];

    let data = Data::from(vec);

    let num_data  = Data::from(0.9);


    let data_pointer = data.into_raw_pointer();
    let recovered_data = Data::from_raw_pointer(data_pointer);

    let data_pointer2 = num_data.into_raw_pointer();
    let recovered_data2 = Data::from_raw_pointer(data_pointer2);

    println!("{:?}", recovered_data.get_type());
    println!("{:?}", recovered_data2.get_type());
}

#[test]
fn arr_test2() {

    let ptr_arr = Data::init_array(5, 4);
    println!("{:?}", Data::set_f64(ptr_arr, 2, 1, 100.1));
    println!("{:?}", Data::set_bool(ptr_arr, 1, 1, true));
    println!("{:?}", Data::set_i8(ptr_arr, 4, 3, 5));

    let owned_arr = Data::from_raw_pointer(ptr_arr);

    let v = owned_arr.into_vec().unwrap();
    let v = v.into_iter().map(|r| r.into_vec()).collect::<Result<Vec<Vec<Data>>, Data>>();

    println!("{:?}", v);
}