use std::borrow::{Borrow, BorrowMut};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn zip_searcher(dir_path: &Path) {
    let mut file_list: Vec<&Path> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let path_buf: PathBuf = path.unwrap().path();

        let (return_path, boo) = is_zip_checker(Box::new(path_buf));

        if boo {
            file_list.push(&**zip_list_pusher(return_path));
        }
    }

    println!("{}", file_list.len());
}

fn is_zip_checker(path: Box<PathBuf>) -> (PathBuf, bool) {
    let pa = path.file_name().unwrap();

    return if pa == OsStr::new("zip") {
        (*path, true)
    } else {
        (*path, false)
    };
}

fn zip_list_pusher(push_target: PathBuf) -> PathBuf {
    push_target
}
