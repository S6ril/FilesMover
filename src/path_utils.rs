use std::{fs::{self, ReadDir}, path::{PathBuf, Path}};
use inquire::{Select};


fn read_dir2vec(paths: ReadDir) -> Vec<String> {
    let mut names = Vec::new();  
    names.push(".".to_string());
    names.push("..".to_string());

    let mut path_names = paths.map(|entry| {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let file_name_as_str = file_name.to_str().unwrap();
        let file_name_as_string = String::from(file_name_as_str);
        file_name_as_string
    }).collect::<Vec<String>>();

    names.append(&mut path_names);
    return names;
}


fn read_path(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    return read_dir2vec(paths);
}


pub fn select_path() -> String {
    let mut path = "./".to_string();
    let mut response: String = "".to_string();

    while response != ".".to_string() + "/" {
        path = path.to_owned() +  &response;
        let names: Vec<String> = read_path(&path);

        let ans: Result<String, inquire::InquireError> = Select::new("Select the path:", names).prompt();
    
        match ans {
            Ok(choice) =>
            {
                response = choice.clone() + "/";
            },
            Err(_) => 
            {
                println!("Exit");
                break;
            },
        }
    }

    path
}


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