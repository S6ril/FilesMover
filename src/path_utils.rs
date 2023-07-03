use std::{fs, path::{PathBuf, Path}};


fn move_file(entry: PathBuf){
        if entry.is_dir() {
            println!("{:?}", entry);

            for file_folder in fs::read_dir(entry).expect("Unable to list") {
                let file_folder = file_folder.expect("unable to get entry").path();
                
                if file_folder.extension().unwrap() == "pdf" {                    
                    let new_name = file_folder.parent().unwrap().parent().unwrap().join(file_folder.file_name().unwrap());
                    fs::rename(file_folder, new_name ).unwrap();

                }
            }
        }
}


pub fn move_and_delete_folder(string_path: String){
    
    let file_path = Path::new(&string_path );

    for entry in fs::read_dir(file_path).expect("unable to get entry") {
        let path = entry.expect("unable to get path").path();
        move_file(path.clone());
        let _ = fs::remove_dir_all(path);
        
    }
} 