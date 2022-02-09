use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

pub fn zip_searcher(dir_path: &Path) {
    let mut file_list: Vec<OsString> = vec![];

    for path in match fs::read_dir(dir_path) {
        Ok(x) => x,
        Err(x) => panic!("{}", x),
    } {
        let mut pa = OsString::new();
        let file_name = Box::new(path.unwrap().file_name());

        file_list.push({
            pa = is_zip_checker(file_name);
            pa
        });
    }

    println!("{}", file_list.len());
}

fn is_zip_checker(p: Box<OsString>) -> OsString {
    let pa = p.clone();

    return if os_string_to_str(*p) == String::from("zip") {
        let string1 = pa.to_os_string();
        string1
    } else {
        let string2 = pa.to_os_string();
        string2
    };
}

fn zip_list_pusher(push_target: &Path) -> &Path {
    push_target
}

fn os_string_to_str(input: OsString) -> String {
    input.into_string().unwrap()
}
