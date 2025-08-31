use directories;
use std::{fmt::format, io, path::PathBuf, process::Output};
use reqwest;
use std::fs::OpenOptions;
use std::io::Write;

const URL:&str = "https://github.com/SharmaDevanshu089/AutoCrate/releases/download/v0.9.7/autocrate-windows-x86-64.zip";
const SRS:&str = "There has been a Serious error with the program please use a different version. Crashing";
const LOGFILE:&str = "install.log";
fn main() {
    let out = String::from(get_directories("tmp").to_string_lossy());
    println!("{}",out);
}
pub fn get_directories(type_of:&str) -> PathBuf{
    let app = directories::ProjectDirs::from("io", "sharmadevanshu089", "autocrate-install").expect(SRS);
    let temp_path = app.cache_dir();
    let error_fix = format!("get directories was called with {}",type_of);
    let mut out = PathBuf::new();
    match type_of {
        "tmp" => out = temp_path.to_owned(),
        _ => log_error(&error_fix),
    }
    return out;
}
fn log_error(text:&str){
    let mut log_file = OpenOptions::new().create(true).append(true).open(LOGFILE).expect("Unable to Create Log File");
    writeln!(&mut log_file,"{}",text).expect("Unable to write a error into log file");
}