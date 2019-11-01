use std::io::prelude::*;
use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs;
// use std::io::ErrorKind;

use crate::template::IPYNB;

pub fn create_notebook(project: String, task: String, base: String, name: String) {
    println!("project: {}, task: {}, notebook: {}", project, task, name);

    // locate the project and task folders, error if it does not exist
    // TODO: Get from PROJ_BASEDIR env var instead of current dir
    // locate the project folder, error if it does not exist
    let mut path = match env::var("PROJ_BASEDIR") {
        Ok(val) => {
            let mut path = PathBuf::from(val);
            path.push(&project);
            if !path.exists() {
                panic!("error creating a new notebook for task {} in {}\nproject \"{}\" does not exist",
                    &task, &project, &project);
            } 
            path.push(&task);
            if !path.exists() {
                panic!("error creating a new notebook for task {} in {}\ntask \"{}\" does not exist",
                    &task, &project, &task);
            }
            path
        },
        Err(e) => {
            panic!("error retrieving the project base directory path: {:?}", e);
        },
    };
    // create a new jupyter notebook inside the specified task
    let filename = format!("{}{}", &name, ".ipynb");
    let filename = filename.as_str();
    path.push(filename);
    let path_str = match path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing notebook path"),
    };
    println!("{}", &path_str);
    let mut file = match fs::File::create(path_str) {
        Ok(file) => file,
        Err(e) => panic!("error creating notebook in {}: {:?}", &path_str, e),
    };

    match file.write_all(IPYNB.as_bytes()) {
        Ok(_) => println!("created notebook in {}", &path_str),
        Err(e) => panic!("error creating notebook in {}: {:?}", &path_str, e),
    }

    // open the browser to the notebook
    let address = format!("{}/{}/{}/{}", &base, &project, &task, &filename);
    match Command::new("open").arg(&address).output() {
        Ok(_) => (),
        Err(e) => panic!("error opening jupyter notebook: {:?}", e),
    };
}