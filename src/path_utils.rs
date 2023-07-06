use std::{fs, path::{PathBuf, Path}};

/// Function to move the file from the child to the parent folder. 
fn move_file2parent(file_folder: PathBuf) {
    let new_name = file_folder.parent().unwrap().parent().unwrap().join(file_folder.file_name().unwrap());
    fs::rename(file_folder, new_name ).unwrap();
}


/// Function to browse all folder and to select a file.
fn search_file(entry: PathBuf, pdf_only: bool){
        if entry.is_dir() {
            println!("{:?}", entry);

            for file_folder in fs::read_dir(entry).expect("Unable to list") {
                let file_folder = file_folder.expect("unable to get entry").path();
                
                if (file_folder.extension().unwrap() == "pdf") & (pdf_only == true) {                    
                    move_file2parent(file_folder);
                }
                else if pdf_only == false {
                    move_file2parent(file_folder);
                }
                // else if pdf_only is check and it is not a pdf, then do nothing
            }
        }
}

/// With a given path, move file in child folder into parent folder and delete the child folder.
/// 
/// ⚠️ This script delete the child folder even if it is not empty. ⚠️
/// 
pub fn move_and_delete_folder(string_path: String, pdf_only: bool){
    
    let file_path = Path::new(&string_path );

    for entry in fs::read_dir(file_path).expect("unable to get entry") {
        let path = entry.expect("unable to get path").path();
        search_file(path.clone(), pdf_only);
        let _ = fs::remove_dir_all(path);
        
    }
} 