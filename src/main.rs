// use std::path::PathBuf;
use structopt::StructOpt;
use std::process::Command;
use std::env;
use std::fs;
use std::io::ErrorKind;


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
}


fn main() {
    match Tsukuru::from_args() {
        Tsukuru::Project {name} => {
            println!("project: {}", &name);

            // retrieve the current directory
            // if successful, append name
            let path = match env::current_dir() {
                Ok(mut v) =>  {
                    v.push(&name);
                    v
                },
                Err(e) => {
                    panic!("error retrieving the path of the current directory: {:?}", e);
                },
            };
            // get the string representation of the dir PathBuf
            // panic if empty
            let path_str = match path.to_str() {
                Some(v) => v,
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
                    Some(v) => v,
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
                Some(v) => v,
                None => panic!("error constructing shared data path"),
            };

            match Command::new("git").arg("init").arg(shared_data_path_str).output() {
                Ok(_) => println!("initialized git version control for shared data in {}", shared_data_path_str),
                Err(e) => panic!("error initializing git version control in shared_data: {:?}", e),
            };

            // create a .project file inside the project folder
            
        },
        Tsukuru::Task {project, name} => {
            println!("project: {}, task: {}", project, name)

            // locate the project folder, error if it does not exist
            // make a task folder inside the specified project folder
            // make data folder inside the task folder
            // git init the data folder
            // make a README.md file
            // create a .task file inside the task folder
        },
        Tsukuru::Notebook {project, task, base: _, name} => {
            println!("project: {}, task: {}, notebook: {}", project, task, name)

            // locate the project and task folders, error if it does not exist
            // create a new jupyter notebook inside the specified task
            // open the browser to the notebook
        },
    };
    // println!("Hello, world!");
}
