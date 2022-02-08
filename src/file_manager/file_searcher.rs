use std::borrow::{Borrow, BorrowMut};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub fn zip_searcher(dir_path: &Path) {
    //let fl_list_for_len = fs::read_dir(dir_path).iter();
    //let file_length = fl_list_for_len.len();

    let mut file_list: Vec<&Path> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let mut path_buf = path.unwrap().path();
        //let rpath = path;

        if is_zip_checker(path_buf.file_name().unwrap()) {
            let path2 = path_buf.as_path();
            let zip_path = zip_list_pusher(&mut path2.borrow());
            file_list.push(zip_path)
        }
    }

    println!("{}", file_list.len());
}

fn is_zip_checker(p: &OsStr) -> bool {
    return if p == OsStr::new("zip") { true } else { false };
}

fn zip_list_pusher<'a>(push_target: &'a mut &Path) -> &'a Path {
    push_target
}
