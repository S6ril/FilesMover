mod path_utils;


fn main() {
    println!("Hello, world!");

    let path = path_utils::select_path();
    // println!("{path}");


    // TODO
    // Maintenant que je selectionne le répertoire literature, il faut naviguer dedans et copier coller les fichiers
    // CF python script

    let string_path = path + "Exported Items/files/";
    path_utils::move_and_delete_folder(string_path);


}
