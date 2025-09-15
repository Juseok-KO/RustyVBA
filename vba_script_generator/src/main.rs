use clap::Parser;
use std::path::PathBuf;
use std::io::Write;

const TEMPLATE_BODY: &'static str = include_str!("template_rusty_vba.bas");
const OUTPUT_VBA_SCRIPT_NAME: &'static str = "rusty_vba.bas";

const ID_DLL_ROOT: &'static str = "{DLL_ROOT}";
const ID_INTERFACE: &'static str = "{INTERFACE}";

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {

    #[arg(short, long)]
    dll_root: String,

    #[arg(short, long)]
    interface: String,
}

fn main() {

    let args = Args::parse();

    let Ok(dll_root) = PathBuf::from(&args.dll_root).canonicalize() else {
        println!("Provided dll-root, {}, does not exist", args.dll_root);
        return 
    };
    let Ok(interface) = PathBuf::from(&args.interface).canonicalize() else {
        println!("Provided interface, {}, does not exist" ,args.interface);
        return
    };

    let dll_root = dll_root.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
    .filter(|e| e != "\\")
    .map(|e| if e.ends_with(":") { String::from("C:") } else { e })
    .collect::<Vec<String>>()
    .join("\\");

    let interface = interface.into_iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
    .filter(|e| e!= "\\")
    .map(|e| if e.ends_with(":") { String::from("C:")} else { e })
    .collect::<Vec<String>>()
    .join("\\");

    let template_body = TEMPLATE_BODY
    .replace(ID_DLL_ROOT, &dll_root)
    .replace(ID_INTERFACE, &interface);

    let mut output_file = match std::fs::File::create_new(OUTPUT_VBA_SCRIPT_NAME) {
        Ok(output_file) => output_file,
        Err(e) => {
            println!("Failed to create output file: {:?}", e);
            return
        }
    };

    match output_file.write_all(template_body.as_bytes()) {
        Ok(_) => {
            println!("New VBA script created")
        }
        Err(e) => {
            println!("Failed to write to output file: {:?}", e)
        }
    }

}
