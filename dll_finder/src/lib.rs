use std::io::{Read, Write};

/// Assumtion / Purpose of the library
/// 1. DLLs can be found from default path where interface.dll file is located, and some explicitly specified paths. 
/// 2. Users will specify paths with a text file which will be stored along with interface.dll
/// 3. Within the text, the users will specifiy a path as a pair of its name and "path", and in the form of name = path, one in one line.
/// 4. This library handles the task of handling the text file.
/// 5. Some functions from this library are called, default path sould be provided. The calling VBA code would do so. 

pub const DEFAULT_PATH: &'static str = "DEFAULT_DIR";
pub const FILE_NAME: &'static str = "dll_paths.txt";
pub const NEW_LINE: char = '\n';
pub const LINE_DELIMITER: char = '=';


/// The text file is created under the specified default_dir
pub fn create_dir_list_file(default_dir: &std::path::Path) -> Result<(), String> {

    let mut file_path = default_dir.to_path_buf();
    file_path.push(FILE_NAME);

    let mut list_file = std::fs::File::create(&file_path)
    .map_err(|e| format!("Failed to create a file at {:?}: {:?}", file_path.display(), e))?;

    let default_dir_str = default_dir.iter().filter_map(|elem| elem.to_str().map(|s| s.to_string()))
    .filter(|e| e != "\\")
    .map(|e| if e.ends_with(":") { String::from("C:")} else { e })
    .collect::<Vec<String>>()
    .join("\\");

    let default_path = format!("{} {} {}{}", DEFAULT_PATH, LINE_DELIMITER, default_dir_str, NEW_LINE);

    list_file.write(default_path.as_bytes())
    .map_err(|e| format!("Failed to write to a file at {:?}: {:?}", file_path.display(), e))?;

    Ok(())
}

fn read_list_text(default_dir: &std::path::Path) -> Result<Vec<Vec<String>>, String> {

    let mut file_path = default_dir.to_path_buf();
    file_path.push(FILE_NAME);

    let mut list_file = std::fs::File::open(&file_path)
    .map_err(|e| format!("Failed to open the DLL path list file at {:?}: {:?}", file_path.display(), e))?;

    let mut file_content = String::new();

    list_file.read_to_string(&mut file_content)
    .map_err(|e| format!("Failed to read the DLL path list file at {:?}: {:?}", file_path.display(), e))?;

    let list = file_content.split(NEW_LINE).filter_map(|line| {
        let mut line_split = line.split(LINE_DELIMITER);
        if let (Some(path_name), Some(path_dir)) = (line_split.next(), line_split.next()) {
            Some(vec![path_name.trim().to_string(), path_dir.trim().to_string()])
        } else {
            None
        }
    }).collect::<Vec<Vec<String>>>();

    if list.is_empty() {
        return Err(format!("Empty dir list"))
    }

    Ok(list)
}

pub fn list_dll_dirs(default_dir: &std::path::Path) -> Result<Vec<String>, String> {

    let all_entries = read_list_text(default_dir)?;

    let dir_names = all_entries.into_iter().filter_map(|entry| {
        if let (Some(name), Some(dir)) = (entry.get(0),entry.get(1)) {
            Some(name.to_string())
        } else {
            None
        }
    }).collect::<Vec<String>>();

    if dir_names.is_empty() {
        return Err(format!("Empty list"))
    }

    Ok(dir_names)
}

pub fn dir_name_to_dir(default_dir: &std::path::Path, dir_name: &str) -> Result<String, String> {

    let all_entries = read_list_text(default_dir)?;

    let mut filtered_dlls = all_entries.into_iter().filter_map(|line| {
        if let (Some(name), Some(dir)) = (line.get(0), line.get(1)) {
            if name.as_str() == dir_name {
                Some(dir.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<String>>();

    if filtered_dlls.len() != 1 {
        return Err(format!("Error 1) Dir with specified name not found, or 2) Too may dirs with the same name"))
    }

    Ok(filtered_dlls.pop().unwrap())
}

pub fn list_dll_under_dir(default_dir: &std::path::Path, dir_name: &str) -> Result<Vec<String>, String> {

    let dir = dir_name_to_dir(default_dir, dir_name).map(|dir| std::path::PathBuf::from(dir))?;
        
    let entries = std::fs::read_dir(&dir)
    .map_err(|e| format!("Failed to read a path '{}': {:?}", dir.display(), e))?;
    
    let dlls = entries.into_iter().filter_map(|entry| {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                    
                if file_name.ends_with(".dll") & !file_name.contains("vba") {
                    Some(file_name.to_string())
                } else {
                    None
                }

            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<String>>();

    if dlls.is_empty() {
        return Err(format!("Empty dir"))
    }

    Ok(dlls)
}

pub fn list_all_dll(default_dir: &std::path::Path) -> Result<Vec<Vec<String>>, String> {

    let dirs = read_list_text(default_dir)?;

    let dir_name_dll_pairs = dirs.into_iter().filter_map(|line| {
        if let (Some(dir_name), Some(_dir)) = (line.get(0), line.get(1)) {

            if let Ok(dlls) = list_dll_under_dir(default_dir, dir_name) {
                Some(dlls.into_iter().map(|dll| vec![dir_name.to_string(), dll])
                .collect::<Vec<Vec<String>>>())

            } else {
                None
            }
        } else {
            None
        }
    }).flatten().collect::<Vec<Vec<String>>>();

    if dir_name_dll_pairs.is_empty() {
        return Err(format!("Empty DLLs list"));
    }

    Ok(dir_name_dll_pairs)
}