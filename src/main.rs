// use std::path::PathBuf;
use structopt::StructOpt;
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
            println!("project: {}", name);

            // make project folder using the name
            // make shared_data, bin, src folders inside the project folder
            // git init the shared_data folder
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
