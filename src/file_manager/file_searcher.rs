use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn zip_searcher(dir_path: &Path) {
    let mut file_list: Vec<&Path> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let mut pa = &path.as_ref().unwrap().path();
        let papa = &path.as_ref().unwrap();
        let papapath = papa.to_owned();
        let path_re = &papapath.path().clone();

        if is_zip_checker(pa) {
            file_list.push(path_re.as_path());
        }
    }

    println!("{}", file_list.len());
}

fn is_zip_checker(p: &PathBuf) -> bool {
    let pa = p.file_name().unwrap();

    return if pa == OsStr::new("zip") { true } else { false };

    //
    // return if pa == OsStr::new("zip") {
    //     (&*pa.into_os_string(), true)
    // } else {
    //     (&*pa.into_os_string(), false)
    // };
}

fn zip_list_pusher(push_target: &Path) -> &Path {
    push_target
}
