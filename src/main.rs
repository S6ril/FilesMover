mod path_utils;

use std::{fs};



fn main() {
    println!("Hello, world!");

    let path = path_utils::select_path();
    // println!("{path}");


    // TODO
    // Maintenant que je selectionne le répertoire literature, il faut naviguer dedans et copier coller les fichiers
    // CF python script

    let file_path: String = path + "Exported Items/files/";

    // println!("{file_path}");

    for entry in fs::read_dir(file_path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry").path();
        
        if entry.is_dir() {
            println!("{:?}", entry);
        }

    }


}
