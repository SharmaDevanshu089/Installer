use directories;
use reqwest::blocking::Response;
use std::{fmt::format, io, path::PathBuf, process::Output};
use reqwest;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::process::exit;
use std::fs::File;

const URL:&str = "https://github.com/SharmaDevanshu089/AutoCrate/releases/download/v0.9.7/autocrate-windows-x86-64.zip";
const SRS:&str = "There has been a Serious error with the program please use a different version. Crashing";
const LOGFILE:&str = "install.log";
fn main() {
    check_cargo();
    download_zip();
}
fn check_cargo(){
    let instruction = "cargo";
    let status = Command::new(instruction).arg("--version").status().expect(SRS);
    if !status.success() {
        println!("Cannot Continue Cargo is not installed or not set to path variable. Please install cargo first");
        exit(0);
    }
}
fn download_zip(){
    let url = URL;
    let mut path =  get_directories("tmp");
    path.push("download.zip");
    let mut file = File::create(path).expect("Unable to Create the donwloaded file");
    let zip = reqwest::blocking::get(url);
    let mut data;
    match zip {
        Ok(zip) => {println!("Sucessfully Donwloaded");data = zip;},
        Err(zip) => {println!("There was a problem with download");log_error(&zip.to_string());exit(0);}
    }
    file.write_all(&data.bytes().expect("I think the data is Corrupted")).expect("Could not write ");
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