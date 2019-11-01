use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;

pub fn create_project(name: String) {
    // TODO: Refactor to be static methods of the enum
    println!("project: {}", &name);

    // TODO: validate name
    // format: Uppercase_first_snake_case

    // TODO: Get from PROJ_BASEDIR env var instead of current dir

    // retrieve the current directory
    // if successful, append name
    let path = match env::var("PROJ_BASEDIR") {
        Ok(val) => {
            let mut path = PathBuf::from(val);
            path.push(&name);
            path
        },
        Err(e) => {
            panic!("error retrieving the project base directory path: {:?}", e);
        },
    };
    // get the string representation of the dir PathBuf
    // panic if empty
    let path_str = match path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing project path"),
    };

    // create project directory
    // if directory already exists, print message and exit
    // panic on error
    match fs::create_dir(path_str) {
        Ok(_) => println!("created project {} in {}", &name, path_str),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("project directory \"{}\" already exists", path_str);
                return;
            },
            _ => panic!("error creating project directory: {:?}", error),
        },
    };

    // make shared_data, bin, src folders inside the project folder
    for folder in ["shared_data", "bin", "src"].iter() {
        let mut path = path.to_owned();
        path.push(folder);
        let path_str = match path.to_str() {
            Some(path_str) => path_str,
            None => panic!("error constructing subdirectory path"),
        };

        match fs::create_dir(path_str) {
            Ok(_) => println!("  created {}", path_str),
            Err(error) => match error.kind() {
                ErrorKind::AlreadyExists => {
                    println!("{} subdirectory already exists", folder);
                    continue;
                },
                _ => panic!("error creating {} subdirectory: {:?}", folder, error),
            } 
        };
    };

    // git init the shared_data folder
    let mut shared_data_path = path.to_owned();
    shared_data_path.push("shared_data");
    let shared_data_path_str = match shared_data_path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing shared data path"),
    };

    match Command::new("git").arg("init").arg(&shared_data_path_str).output() {
        Ok(_) => println!("initialized git version control for shared data in {}", shared_data_path_str),
        Err(e) => panic!("error initializing git version control in shared_data: {:?}", e),
    };

    // create a .project file inside the project folder
    let marker_name = ".project";
    let mut marker_path = path.to_owned();
    marker_path.push(marker_name);
    let marker_path_str = match marker_path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing {} path", marker_name),
    };
    match fs::File::create(marker_path_str) {
        Ok(_) => println!("created {} file in {}", marker_name, &path_str),
        Err(e) => panic!("error creating {} file in {}: {:?}", marker_name, &path_str, e),
    };
}

