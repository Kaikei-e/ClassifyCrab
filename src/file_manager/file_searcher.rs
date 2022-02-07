use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub fn zip_searcher(dir_path: &Path){
    let mut file_list: Vec<&OsStr>  = vec![];

    for path in fs::read_dir(dir_path).unwrap() {
        let path = path.unwrap().path();
        let extension = path.extension().unwrap();

        if extension == "zip" {
            file_list.push(*path.file_name())
        }
    }

    println!(file_list.len())


}