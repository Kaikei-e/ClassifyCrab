use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

pub fn zip_searcher(dir_path: &Path) {
    let mut file_list: Vec<&Path> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let path = path.unwrap();
        let mut pa = path.path().as_os_str();

        if pa == OsStr::new("zip") {
            file_list.push(pa.as_ref());
        }
    }

    println!("{}", file_list.len());
}

fn is_zip_checker(p: &PathBuf) -> (&OsStr, bool) {
    let pa = p.file_name().unwrap();

    return if pa == OsStr::new("zip") {
        (&pa, true)
    } else {
        (&pa, false)
    };
}

fn zip_list_pusher(push_target: &Path) -> &Path {
    push_target
}
