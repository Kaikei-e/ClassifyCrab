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
        let mut pa = PathBuf::new();
        let file_name = path.unwrap().file_name();
        //let papa = &path.as_ref().unwrap();
        //let papapath = papa.to_owned();
        //let path_re = &papapath.path().clone();
        file_list.push(is_zip_checker(file_name).as_ref());
        //
        // if is_zip_checker(pa) {
        //     file_list.push(path_re.as_path());
        // }
    }

    println!("{}", file_list.len());
}

//noinspection ALL
fn is_zip_checker(p: OsString) -> OsString {
    //let pa = p;

    return if os_string_to_str(p).unwrap().to_string() == String::from("zip") {
        let string1 = p.to_os_string();
        string1
    } else {
        let string2 = p.to_os_string();
        string2
    };

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

fn os_string_to_str(input: OsString) -> Option<&'static str> {
    input.to_str()
}
