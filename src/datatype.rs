use crate::Pointer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeCode {
    I8 = 1,
    I16 = 2,
    I32 = 3,
    I64 = 4,
    F32 = 5,
    F64 = 6,
    BOOL = 7,
    STRING = 8,
    ARRAY = 9,
}

#[derive(Debug, Clone)]
pub enum Value {
    I8(i8), 
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    BOOL(bool),
    STRING(String),
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

    pub fn get_i8(pointer: *mut Pointer) -> Result<i8, String> {
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

    pub fn get_i16(pointer: *mut Pointer) -> Result<i16, String> {
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

    pub fn get_i32(pointer: *mut Pointer) -> Result<i32, String> {
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

    pub fn get_i64(pointer: *mut Pointer) -> Result<i64, String> {
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

    pub fn get_f32(pointer: *mut Pointer) -> Result<f32, String> {
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

    pub fn get_f64(pointer: *mut Pointer) -> Result<f64, String> {
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

    pub fn get_bool(pointer: *mut Pointer) -> Result<bool, String> {
        let reference  = unsafe { &*(pointer as *mut Data) as &Data };
        match reference.get_type() {
            TypeCode::ARRAY => {
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