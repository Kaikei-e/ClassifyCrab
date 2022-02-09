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
        //let papa = &path.as_ref().unwrap();
        //let papapath = papa.to_owned();
        //let path_re = &papapath.path().clone();
        file_list.push({pa = is_zip_checker(file_name);
        pa
        });
        //
        // if is_zip_checker(pa) {
        //     file_list.push(path_re.as_path());
        // }
    }

    println!("{}", file_list.len());
}

//noinspection ALL
fn is_zip_checker(p: Box<OsString>) -> OsString {
    let pa = p.clone();

    return if os_string_to_str(*p) == String::from("zip") {
        let string1 = pa.to_os_string();
        string1
    } else {
        let string2 = pa.to_os_string();
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

fn os_string_to_str(input: OsString) -> String {
    input.into_string().unwrap()
}
