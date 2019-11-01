use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;

pub fn create_task(project: String, name: String) {
    println!("project: {}, task: {}", project, name);

    // TODO: Get from PROJ_BASEDIR env var instead of current dir
    // locate the project folder, error if it does not exist
    let path = match env::var("PROJ_BASEDIR") {
        Ok(val) => {
            let mut path = PathBuf::from(val);
            path.push(&project);
            if path.exists() {
                path.push(&name);
            } else {
                panic!("error creating a new task for project {}\nproject \"{}\" does not exist",
                    &project, &project);
            }
            path
        },
        Err(e) => {
            panic!("error retrieving the project base directory path: {:?}", e);
        },
    };
    // make a task folder inside the specified project folder
    let path_str = match path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing task path"),
    };
    match fs::create_dir(path_str) {
        Ok(_) => println!("created task {} for project {} in {}", &name, &project, path_str),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("task directory \"{}\" already exists", path_str);
                return;
            },
            _ => panic!("error creating task directory in project {}: {:?}", &project, error),
        },
    };

    // make data folder inside the task folder
    let mut data_path = path.to_owned();
    data_path.push("data");
    let data_path_str = match data_path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing data subdirectory path"),
    };
    match fs::create_dir(data_path_str) {
        Ok(_) => println!("  created {}", data_path_str),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("data subdirectory already exists");
            },
            _ => panic!("error creating data subdirectory: {:?}", error),
        } 
    };
    
    // git init the data folder
    match Command::new("git").arg("init").arg(&data_path_str).output() {
        Ok(_) => println!("initialized git version control for data in {}", data_path_str),
        Err(e) => panic!("error initializing git version control in data: {:?}", e),
    };

    // make a README.md file
    let mut readme_path = path.to_owned();
    readme_path.push("README.md");
    let readme_path_str = match readme_path.to_str() {
        Some(path_str) => path_str,
        None => panic!("error constructing README.md path"),
    };
    match fs::File::create(readme_path_str) {
        Ok(_) => println!("created README.md file in {}", &readme_path_str),
        Err(e) => panic!("error creating README.md  file in {}: {:?}", &readme_path_str, e),
    };

    // create a .task file inside the task folder
    let marker_name = ".task";
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