#[test]
fn test() {
    println!("This is a test")
}

pub mod datatype;

#[cfg(feature="from-vba")]
pub mod from_vba;


#[derive(Debug, Clone, Copy)]
pub struct Pointer;