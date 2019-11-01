use structopt::StructOpt;

mod template;
mod project;
mod task;
mod notebook;



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
            // Create a new project directory and initialize
            project::create_project(name);
        },
        Tsukuru::Task {project, name} => {
            // TODO: Validate if project exists
            // Create a new task within the project and initialize
            task::create_task(project, name);
        },
        Tsukuru::Notebook {project, task, base, name} => {
            // TODO: Validate if project exists
            // TODO: Validate if task exists within the given project
            // Create a new notebook within the specified project and task, then initialize
            notebook::create_notebook(project, task, base, name);        
        },
        Tsukuru::Config {proj_base_dir} => {
            println!("proj_base_dir: {}", proj_base_dir);

            // TODO: set proj_base_dir env var 
        },
    };
    // println!("Hello, world!");
}
