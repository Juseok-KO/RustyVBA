#[test]
fn test() {
    println!("This is a test")
}

pub mod datatype;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pointer;

unsafe impl Send for Pointer {}