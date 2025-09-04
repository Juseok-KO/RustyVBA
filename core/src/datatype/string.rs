use crate::Pointer;

#[repr(C)]
#[derive(Debug)]
pub struct PtrCStr(*const u8);

#[repr(C)]
#[derive(Debug)]
pub struct CSTRING(pub(crate) PtrRustCreatedCStr);

#[repr(C)]
#[derive(Debug)]
pub(crate) struct PtrRustCreatedCStr {
    size: usize,
    capacity: usize,
    pub(crate) ptr: *const u8,
}

impl Iterator for PtrCStr {
    type Item = u8;

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

    let ptr = PtrCStr(ptr_cstr as *const u8);
    String::from_utf8(ptr.into_iter().collect::<Vec<u8>>())
    .map_err(|e| e.to_string())
}

impl From<String> for CSTRING {
    fn from(value: String) -> Self {
        let mut vector = value.bytes().collect::<Vec<u8>>();
        vector.push(0);
        CSTRING(PtrRustCreatedCStr { size: vector.len(), capacity: vector.capacity(), ptr: vector.leak() as *mut [u8] as *mut u8})
    }
}

impl CSTRING {
    pub(crate) fn into_string(self) -> Result<String, String> {
        let mut vector = unsafe { Vec::from_raw_parts(self.0.ptr as *mut u8, self.0.size, self.0.capacity) };
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