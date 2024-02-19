extern crate colored;

use chrono::DateTime;
use chrono::offset::Utc;

use colored::Colorize;

use std::fs::*;
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::process::exit;

use crate::util;

pub struct LocalFile {
    pub name:   String,
    pub time:   String,
    pub owner:  String,
    pub mode:   u32,
    pub size:   u64,
    pub is_dir: bool
}

pub fn list_files(path: &str) -> Vec<LocalFile> {
    let mut files: Vec<LocalFile> = Vec::new();
    let entries: Result<ReadDir, Error> = read_dir(path);

    match entries {
        Ok(entries)=> {
            for path in entries {
                let entry: DirEntry = path.unwrap();
                let meta: Metadata = entry.metadata().unwrap();

                let mut date_mod: String = String::new();
                if let Ok(date_time) = meta.modified() {
                    let dt: DateTime<Utc> = date_time.into();
                    date_mod.push_str(format!("{}", dt.format("%H:%M:%S %d/%m/%Y")).as_str());
                }
                else {
                    date_mod.push_str("[date error]");
                }

                let mut fileowner: String = String::new();
                if let Some(owner) = util::sys_username(meta.uid()) {
                    fileowner = owner;
                }

                let mut actual_name: String = entry.path().display().to_string();
                if let Some(filename) = actual_name.split('/').last() {
                    actual_name = filename.to_string();
                }

                files.push(LocalFile{
                    name: actual_name,
                    mode: meta.permissions().mode(),
                    owner: fileowner,
                    time: date_mod,
                    size: meta.len(),
                    is_dir: meta.is_dir()
                });
            }
        },
        Err(ref _error)=> {
            eprintln!("Something went {}.", "wrong".red().bold());
            exit(0);
        }
    }

    files
}

pub fn print_file(file: LocalFile) {
    let filesize = if !file.is_dir {
        file.size.to_string()
    }
    else {
        "-".to_string()
    };

    println!("{} {} {} [{}\t] {}",
        util::mode_str(file.mode).green(),
        file.owner.red(),
        file.time.to_string().cyan().italic(),
        util::pad_str(filesize.to_string(), 14),
        file.name.bold());
}