pub const RUST_LOGO_PNG: &[u8] = include_bytes!("rust-logo-64x64-blk.png");
pub const CUSTOM_UI_DIR: &'static str = "CustomUI";
pub const CUSTOM_UI_FILE: &'static str  = "rusty_vba_ui.xml";
pub const CUSTOM_UI_BODY_TEMPLATE: &'static str = include_str!("rusty_vba_ui.xml");

pub const RELS_FILE: &'static str = ".rels";
pub const DIR_RELS: &'static str = "_rels";
pub const CUSTOM_UI_REL: &'static str = "<Relationship Id=\"rusty_vba_ui_rel\" Type=\"http://schemas.microsoft.com/office/2006/relationships/ui/extensibility\" Target=\"{}/{}\"/>";
pub const CLOSING_RELS: &'static str = "</Relationships>";

pub const IMAGE_DIR: &'static str = "images";
pub const IMAGE_REL_FILE: &'static str = "rusty_vba_ui.xml.rels";
pub const IMAGE_REL_BODY_TEMPLATE: &'static str = include_str!("rusty_vba_icon_rels.xml");
pub const IMAGE_ID: &'static str = "rust_icon";
pub const IMAGE_FILE: &'static str = "rust_logo.png";

pub const UI_ACTION_INIT_FUNCS: &'static str = "RibbonInitFuncs";
pub const UI_ACTION_INIT_RESOURCES: &'static str = "RibbonInitResources";
pub const UI_ACTION_DROP_RESOURCES: &'static str = "RibbonDropResources";

pub const FILE_CONTENT_TYPE: &'static str = "[Content_Types].xml";
pub const IMAGE_FILE_EXT: &'static str = "png";
pub const ADDITIONAL_BODY_CONTENT_TYPE_TEMPLATE: &'static str = "<Default Extension=\"{EXT}\" ContentType=\"image/{EXT}\"/>";

use std::{io::{Read, Write}, path::Path};
use zip::{ZipWriter, write::SimpleFileOptions};

fn icon_relation_body() -> String {

    IMAGE_REL_BODY_TEMPLATE
    .replacen("{}", IMAGE_ID, 1)
    .replacen("{}", IMAGE_DIR, 1)
    .replacen("{}", IMAGE_FILE, 1)
}

fn custom_ui_body() -> String {

    CUSTOM_UI_BODY_TEMPLATE
    .replace("{IMAGE}", IMAGE_ID)
    .replacen("{}", UI_ACTION_INIT_FUNCS, 1)
    .replacen("{}", UI_ACTION_INIT_RESOURCES, 1)
    .replacen("{}", UI_ACTION_DROP_RESOURCES, 1)
}

fn update_rels_body(org: String) -> Result<String, String> {

    let Some(bf) = org.split(CLOSING_RELS).next() else {
        return Err(format!("Failed to parse the .rels"))
    };

    let mut bf = bf.to_string();
    let custom_ui_rel = CUSTOM_UI_REL.replacen("{}", CUSTOM_UI_DIR,1).replacen("{}", CUSTOM_UI_FILE, 1);
    bf.push_str(&custom_ui_rel);
    bf.push_str(CLOSING_RELS);

    Ok(bf)
}

fn content_type_new_line() -> String {
    ADDITIONAL_BODY_CONTENT_TYPE_TEMPLATE
    .replace("{EXT}", IMAGE_FILE_EXT)
}

fn update_content_type_body(org: String) -> String {

    const MAX_COUNTER: usize  = 2;
    const FLAG: char = '>';

    let mut counter = 0;
    
    let mut new_body = String::new();

    for c in org.chars() {

        new_body.push(c);

        if c == FLAG {
            counter += 1;
        }

        if counter == MAX_COUNTER {
            new_body.push_str(&content_type_new_line());
            counter += 1;
        }
    }

    new_body
}

pub fn set_custom_ui(base_dir: &Path) -> Result<(), String>{

    let mut dir_custom_ui = base_dir.to_path_buf();
    dir_custom_ui.push(CUSTOM_UI_DIR);

    std::fs::create_dir(&dir_custom_ui)
    .map_err(|e| format!("Failed to create a folder at {}: {:?}", dir_custom_ui.display(), e))?;

    let mut dir_custom_ui_xml = dir_custom_ui.to_path_buf();
    dir_custom_ui_xml.push(CUSTOM_UI_FILE);

    let mut custom_ui_file = std::fs::File::create(&dir_custom_ui_xml)
    .map_err(|e| format!("Failed to create a file at {}: {:?}", dir_custom_ui_xml.display(), e))?;

    custom_ui_file.write_all(custom_ui_body().as_bytes())
    .map_err(|e| format!("Failed to write to a file at {}: {:?}", dir_custom_ui_xml.display(), e))?;

    let mut dir_custom_ui_icon = dir_custom_ui.to_path_buf();
    dir_custom_ui_icon.push(IMAGE_DIR);

    std::fs::create_dir(&dir_custom_ui_icon)
    .map_err(|e| format!("Failed to create a folder at {}: {:?}", dir_custom_ui_icon.display(), e))?;

    dir_custom_ui_icon.push(IMAGE_FILE);

    let mut custom_icon_file = std::fs::File::create(&dir_custom_ui_icon)
    .map_err(|e|format!("Failed to create a file at {}: {:?}", dir_custom_ui_icon.display(), e))?;

    custom_icon_file.write_all(RUST_LOGO_PNG)
    .map_err(|e| format!("Failed to wirte to a file at {}: {:?}", dir_custom_ui_icon.display(), e))?;

    let mut dir_custom_ui_rel = dir_custom_ui.to_path_buf();
    dir_custom_ui_rel.push(DIR_RELS);

    std::fs::create_dir(&dir_custom_ui_rel)
    .map_err(|e| format!("Failed to create a folder at {}: {:?}", dir_custom_ui_rel.display(), e))?;

    dir_custom_ui_rel.push(IMAGE_REL_FILE);

    let mut custom_ui_rel_file = std::fs::File::create(&dir_custom_ui_rel)
    .map_err(|e| format!("Failed to create a file at {}: {:?}", dir_custom_ui_rel.display(), e))?;

    custom_ui_rel_file.write_all(icon_relation_body().as_bytes())
    .map_err(|e| format!("Failed to write to a file at {}: {:?}", dir_custom_ui_rel.display(), e))?;

    Ok(())

}

pub fn update_content_type(base_dir: &Path) -> Result<(), String> {

    let mut dir_org_content_type = base_dir.to_path_buf();
    dir_org_content_type.push(FILE_CONTENT_TYPE);

    let mut org_content_type_file = std::fs::File::open(&dir_org_content_type)
    .map_err(|e| format!("Failed to open a file at {}: {:?}", dir_org_content_type.display(), e))?;

    let mut org_content_type_body = String::new();
    org_content_type_file.read_to_string(&mut org_content_type_body)
    .map_err(|e| format!("Failed to read a file at {}: {:?}", dir_org_content_type.display(), e))?;

    drop(org_content_type_file);

    let updated_content_type_body = update_content_type_body(org_content_type_body);

    let mut dir_tmp_content_type = dir_org_content_type.to_path_buf();

    dir_tmp_content_type.set_extension("tmp");

    let mut tmp_content_type_file = std::fs::File::create(&dir_tmp_content_type)
    .map_err(|e| format!("Failed to creata a file at {}: {:?}", dir_tmp_content_type.display(), e))?;

    tmp_content_type_file.write_all(updated_content_type_body.as_bytes())
    .map_err(|e| format!("Failed to write to a file at {}: {:?}", dir_tmp_content_type.display(), e))?;

    std::fs::rename(&dir_tmp_content_type, &dir_org_content_type)
    .map_err(|e| format!("Failed to move a file from {} to {}: {:?}", dir_tmp_content_type.display(), dir_org_content_type.display(), e))

}

pub fn update_rels(base_dir: &Path) -> Result<(), String> {

    let mut dir_org_rels = base_dir.to_path_buf();
    dir_org_rels.push(DIR_RELS);
    dir_org_rels.push(RELS_FILE);

    let mut org_rels_file = std::fs::File::open(&dir_org_rels)
    .map_err(|e| format!("Failed to open a file at {}: {:?}", dir_org_rels.display(), e))?;

    let mut org_rels_body = String::new();
    org_rels_file.read_to_string(&mut org_rels_body)
    .map_err(|e| format!("Failed to read a file at {}: {:?}", dir_org_rels.display(), e))?;

    drop(org_rels_file);

    let updated_rels_body = update_rels_body(org_rels_body)?;

    let mut dir_tmp_rels = dir_org_rels.to_path_buf();
    dir_tmp_rels.set_extension("tmp");

    let mut tmp_rels_file = std::fs::File::create(&dir_tmp_rels)
    .map_err(|e| format!("Failed to create a file at {}: {:?}", dir_tmp_rels.display(), e))?;

    tmp_rels_file.write_all(updated_rels_body.as_bytes())
    .map_err(|e| format!("Failed to write to a file at {}: {:?}", dir_tmp_rels.display(), e))?;

    std::fs::rename(&dir_tmp_rels, &dir_org_rels)
    .map_err(|e| format!("Failed to move a file from {} to {}: {:?}", dir_tmp_rels.display(), dir_org_rels.display(), e))

}

pub fn drop_root(root: &Path, dir: &Path) -> Result<String, String> {

    let mut root_iter = root.iter();
    let mut dir_iter = dir.iter();

    let mut dir_tail = Vec::new();

    loop {
        match (root_iter.next(), dir_iter.next()) {
            (Some(_1), Some(_2)) => {},
            (None, Some(d)) => {
                dir_tail.push(d);
            }
            (None, None) => {
                break;
            } 
            _ => {
                return Err(format!("Unexpected dir provided: {}", dir.display()))
            }
        }
    }
    
    Ok(dir_tail.iter().map(|e| e.to_str().map_or(Err(format!("Failed to convert dir into str")), | s| Ok(s.to_string())))
    .collect::<Result<Vec<String>, String>>()?
    .join("/"))
    
}

pub fn write_to_zip(local_root_dir: &Path, new_archive: &mut ZipWriter<std::fs::File>, start_dir: &Path) -> Result<(), String> {

    let mut dir_reader = std::fs::read_dir(start_dir)
    .map_err(|e| format!("Failed to read_dir: {}: {:?}", start_dir.display(), e))?;

    while let Some(Ok(d)) = dir_reader.next() {

        let local_dir = d.path();
        let zipped_dir = drop_root(local_root_dir, &local_dir)?;

        if local_dir.is_dir() {

            new_archive.add_directory(zipped_dir.as_str(), SimpleFileOptions::default())
            .map_err(|e| format!("Failed to add a dir to zip: {}: {:?}", zipped_dir, e))?;
            write_to_zip(local_root_dir, new_archive, &local_dir)?;
        
        } else if local_dir.is_file() {

            let mut local_file = std::fs::File::open(&local_dir)
            .map_err(|e| format!("Failed to open a file at {}: {:?}", local_dir.display(), e))?;
            
            let mut local_file_bytes = Vec::new();
            local_file.read_to_end(&mut local_file_bytes)
            .map_err(|e| format!("Failed to write to a file at {}: {:?}", local_dir.display(), e))?;

            new_archive.start_file(&zipped_dir, SimpleFileOptions::default())
            .map_err(|e| format!("Failed to add a file to zip: {}: {:?}", zipped_dir, e))?;
            
            new_archive.write_all(&local_file_bytes)
            .map_err(|e| format!("Failed to write to zipped file at {}: {:?}", zipped_dir, e))?;

            new_archive.flush()
            .map_err(|e| format!("Failed to flush memory to zipped file: {:?}", e))?;
        }

    }

    Ok(())
}