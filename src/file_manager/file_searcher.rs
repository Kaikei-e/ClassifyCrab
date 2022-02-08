use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub fn zip_searcher(dir_path: &Path) {
    let mut file_list: Vec<&Path> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let path_clone = path.clone().expect("Failed to clone file path.");
        let mut path_buf = path.unwrap().path();
        //let rpath = path;

        if is_zip_checker(path_buf.file_name().unwrap()) {
            file_list.push(path_clone.unwrap().path().borrow());
        }
    }

    println!("{}", file_list.len());
}

fn is_zip_checker(p: &OsStr) -> bool {
    return if p == OsStr::new("zip") { true } else { false };
}

fn zip_list_pusher(push_target: Box<Path>) -> Box<Path> {
    push_target
}
