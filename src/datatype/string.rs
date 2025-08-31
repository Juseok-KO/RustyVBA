use crate::Pointer;

#[derive(Debug)]
pub(super) struct PtrCStr(*const u8);

#[derive(Debug, Clone)]
pub(crate) struct CSTRING(Vec<u8>);

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