use clap::Parser;

use vba_ribbon_creator::{set_custom_ui, update_content_type, update_rels, write_to_zip};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {

    #[arg(short, long)]
    path_excel: String
}

fn main() {

    let args = Args::parse();

    let path_excel = std::path::PathBuf::from(&args.path_excel);

    path_excel.is_file().then(||Some(())).expect("The provided path is not a file");

    let mut path_excel_w_ribbon = path_excel.parent().unwrap().to_path_buf();
    path_excel_w_ribbon.push("RustyVBARibbonTmp");

    let zipfile = std::fs::File::open(&path_excel).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();
    archive.extract(&path_excel_w_ribbon).unwrap();
    
    drop(archive);

    update_rels(&path_excel_w_ribbon).unwrap();
    update_content_type(&path_excel_w_ribbon).unwrap();
    set_custom_ui(&path_excel_w_ribbon).unwrap();

    let mut new_zipped_file_dir = path_excel.to_path_buf();
    new_zipped_file_dir.add_extension("tmp").then(||Some(()))
    .expect("Failed to create the name for the temp zip");

    let zipped_file = std::fs::File::create(&new_zipped_file_dir)
    .map_err(|e| format!("Failed to create a file at {}: {:?}", new_zipped_file_dir.display(), e)).unwrap();

    let mut new_archive = zip::ZipWriter::new(zipped_file);

    write_to_zip(&path_excel_w_ribbon, &mut new_archive, &path_excel_w_ribbon).unwrap();

    drop(new_archive);

    std::fs::rename(&new_zipped_file_dir, &path_excel).unwrap();

    std::fs::remove_dir_all(&path_excel_w_ribbon).unwrap();

    println!("ENd");
}