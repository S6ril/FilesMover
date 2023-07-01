use std::{fs::{self, ReadDir}};
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