use std::env;
use std::{fs, path};
use copy_dir::copy_dir;
use sevenz_rust;

pub fn init(dir_name: &str, pass: &String){
    sevenz_rust::compress_to_path_encrypted(dir_name,
        format!("{dir_name}.7z"), 
        pass.as_str().into())
    .unwrap();
}
