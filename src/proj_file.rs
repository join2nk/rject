use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

fn proj_path() -> PathBuf {
    // create path
    let home = match dirs::home_dir() {
        None => panic!("Cannot find home dir"),
        Some(h) => h,
    };
    // join $HOME and .proj
    let path = Path::new("").join(home).join(".proj");
    path.to_path_buf()
}

pub fn read_proj() -> Vec<String> {
    // open or create
    let file = match File::open(proj_path()) {
        // proj file doesn't exist
        Err(_) => match File::create(proj_path()) {
            // can't create
            Err(e) => panic!("issue reading or creating file: {}", e),
            // empty file created (no need to read)
            Ok(_) => return vec![],
        },

        // return file
        Ok(file) => file,
    };

    // get buffer from file
    let buf = BufReader::new(file);
    let mut rewrite = false;
    // convert buffer to vector with no empty entries
    let projs: Vec<String> = buf
        .lines()
        .map(|l| l.expect("file cannot be read"))
        .filter(|p| !p.is_empty()) // no blank lines
        .filter(|p| { // check for currupted paths
            if !Path::new(p).exists() {
                rewrite = true;
                return false;
            }
            return true;
        })
        .collect();

    // if proj file needs to be re written
    if rewrite {
        let mut file = File::create(proj_path()).expect("cannot access proj file");

        // re write file from scratch
        for p in projs.iter() {
            file.write_fmt(format_args!("{}\n", p))
                .expect("cannot write");
        }
    }

    // return vector of projects
    projs
}

pub fn add_project(p: &str, projs: &[String]) -> Option<String> {
    // make sure project isn't already present
    if projs.contains(&p.to_string()) {
        return Some("Project Already Listed".to_string());
    }
    // open file for appending
    let mut file = OpenOptions::new()
        .append(true)
        .open(proj_path())
        .expect("cannot open proj file");

    let res = match Path::new(p).exists() {
        // append new project
        true => {
            file.write_fmt(format_args!("{}\n", p))
                .expect("wrote to file");
            None
        }
        // path doesn't exist
        false => Some("Path Does Not Exists.".to_string()),
    };
    // send information where function is called
    res
}

pub fn remove_project(proj: &str, projs: Vec<String>) -> Vec<String> {
    // open file (overwrite)
    let mut file = File::create(proj_path()).expect("cannot access proj file");

    // remove specified project
    let new_projs: Vec<String> = projs.into_iter().filter(|p| p != proj).collect();

    // re write file from scratch
    for p in new_projs.iter() {
        file.write_fmt(format_args!("{}\n", p))
            .expect("cannot write");
    }

    // return refactored list of projects
    new_projs
}
