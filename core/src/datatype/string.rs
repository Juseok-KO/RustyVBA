use crate::Pointer;

#[repr(C)]
#[derive(Debug)]
pub struct PtrVBAStr(*const u16);

#[repr(C)]
#[derive(Debug)]
pub struct CSTRING(pub(crate) Vec<u8>);

impl Iterator for PtrVBAStr {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {

        if unsafe { *self.0} == 0 {
            None
        
        } else {

            let val_to_return = unsafe { *self.0 };
            self.0 = unsafe { self.0.offset(1) };
            Some(val_to_return)
        }
    }
}

pub fn copy_from_cstr(ptr_cstr: *const Pointer) -> Result<String, String> {

    let ptr = PtrVBAStr(ptr_cstr as *const u16);
    String::from_utf16(ptr.into_iter().collect::<Vec<u16>>().as_ref())
    .map_err(|e| e.to_string())
}

impl From<String> for CSTRING {
    fn from(value: String) -> Self {
        let mut vector = value.bytes().collect::<Vec<u8>>();
        vector.push(0);
        CSTRING(vector)
    }
}

impl CSTRING {
    pub(crate) fn into_string(self) -> Result<String, String> {
        let mut vector = self.0;
        vector.pop();
        String::from_utf8(vector)
        .map_err(|e| e.to_string())
    }
}

#[test]
fn string_drop_test() {

    let cstr = crate::datatype::Data::from(CSTRING::from(String::from("ユニコードも問題ないはず"))).into_raw_pointer();
    println!("{:?}", cstr);
    let ptr_str = crate::datatype::Data::get_ptr_str(cstr);
    println!("{:?}", ptr_str);
    println!("{:?}", cstr);
    crate::datatype::Data::drop(cstr);
}

#[test]
fn array_test() {
    use crate::datatype::Data;
    let ptr_arr = Data::from(vec![
        Data::from(vec![Data::from(CSTRING::from(String::from("Hello, this is Rust"))), Data::from(100_i8), Data::from(true)])
        ]
    ).into_raw_pointer();

    println!("num_rows: {:?}", Data::get_arr_row(ptr_arr));
    println!("num_cols: {:?}", Data::get_arr_col(ptr_arr));

}