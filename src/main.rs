use std::io::prelude::*;
use structopt::StructOpt;
use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::ErrorKind;

// TODO: Create templates for different kernels
static IPYNB: &'static str = 
r#"{
 "cells": [
  {
   "cell_type": "code",
   "execution_count": null,
   "metadata": {},
   "outputs": [],
   "source": []
  }
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Python 3",
   "language": "python",
   "name": "python3"
  },
  "language_info": {
   "codemirror_mode": {
    "name": "ipython",
    "version": 3
   },
   "file_extension": ".py",
   "mimetype": "text/x-python",
   "name": "python",
   "nbconvert_exporter": "python",
   "pygments_lexer": "ipython3",
   "version": "3.7.4"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 2
}"#;

#[derive(Debug, StructOpt)]
#[structopt(name = "tsukuru", about = "Create new projects and notebooks")]
enum Tsukuru {
    /// List, create, or delete projects
    Project {
        /// Project name
        name: String,
    },
    /// List, create, or delete project tasks
    Task {
        /// Which project to create the task in
        #[structopt(short, long)]
        project: String,

        /// Task name
        name: String,
    },
    /// Create new Jupyter notebooks
    Notebook {
        /// Which project to create the notebook in
        #[structopt(short, long)]
        project: String,

        // TODO: Make --project and --task optional,
        // but require each other when one of them is used

        /// Which task to create the notebook in
        #[structopt(short, long, requires("project"))]
        task: String,

        #[structopt(long, default_value = "http://localhost:8888/tree")]
        base: String,

        /// Notebook name
        name: String,
    },

    /// Configure Tsukuru
    Config {
        // TODO: Get default value from environment

        /// Base directory
        #[structopt(long="proj_basedir")]
        proj_base_dir: String,

        
    },
}


fn main() {
    match Tsukuru::from_args() {
        Tsukuru::Project {name} => {

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
        },
        Tsukuru::Task {project, name} => {
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
        },
        Tsukuru::Notebook {project, task, base, name} => {
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
        },

        Tsukuru::Config {proj_base_dir} => {
            println!("proj_base_dir: {}", proj_base_dir);

            // TODO: set proj_base_dir env var 
        },
    };
    // println!("Hello, world!");
}
