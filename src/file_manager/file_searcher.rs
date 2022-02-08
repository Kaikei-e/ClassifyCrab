use std::ffi::OsStr;
use std::fmt::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn zip_searcher(dir_path: &Path) {
    //let fl_list_for_len = fs::read_dir(dir_path).iter();
    //let file_length = fl_list_for_len.len();

    let mut file_list: Vec<&Path> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let path_buf = path.unwrap().path();
        let path = path;

        //let extension = path.extension().unwrap();
        if is_zip_checker(path_buf.file_name().unwrap()) {
            let rpath = path.unwrap().path();
            file_list.push(&*rpath);
        }
    }

    println!("{}", file_list.len())
}

fn is_zip_checker(p: &OsStr) -> bool {
    return if p == OsStr::new("zip") { true } else { false };
}
