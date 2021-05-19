use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand};
use std::fs::{create_dir, File};
use std::io::prelude::*;
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";
fn main() {
    let ptype = cargs();
    if let Some(project_type) = ptype {
        project_type.create();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProjectType {
    Bin(String),
    Lib(String),
}

impl ProjectType {
    fn create(&self) {
        match self {
            Self::Bin(name) => create_bin_project(name),
            Self::Lib(name) => create_lib_project(name),
        }
    }
}

fn create_basic_dir(name: &str) {
    create_dir(name).expect("Could not create Poject Folder.");
    create_dir(format!("{}/src", name)).expect("Could not create src Folder.");
    std::process::Command::new("git")
        .args(&["init", name])
        .spawn()
        .expect("Creating a git repository Failed.");
    let mut file =
        File::create(format!("{}/README.md", name)).expect("Could not create __main__.py");
    let boilerplate = format!("# {}", name);
    file.write_all(boilerplate.as_bytes())
        .expect("Failed to write to __main__.py");
}

fn create_bin_project(name: &str) {
    create_basic_dir(name);
    let mut file =
        File::create(format!("{}/src/__main__.py", name)).expect("Could not create __main__.py");
    let boilerplate =
        "def main():\n    print(\"Hello World\")\n\nif __name__ == \"__main__\":\n    main()"
            .to_string();
    file.write_all(boilerplate.as_bytes())
        .expect("Failed to write to __main__.py");
}

fn create_lib_project(name: &str) {
    create_basic_dir(name);
    let _ =
        File::create(format!("{}/src/__init__.py", name)).expect("Could not create __main__.py");
}

fn cargs() -> Option<ProjectType> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(AUTHOR)
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("new")
                .about("Used for configuration")
                .arg(
                    Arg::with_name("project_name")
                        .help("Set name of project")
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("binary")
                        .long("bin")
                        .help("Makes a file structure with a __main__.py in src folder.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("library")
                        .long("lib")
                        .help("Makes a library file structure with a __init__.py in src folder.")
                        .takes_value(false),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches
            .value_of("project_name")
            .unwrap_or("default")
            .to_string();
        if matches.is_present("library") {
            return Some(ProjectType::Lib(project_name));
        }
        return Some(ProjectType::Bin(project_name));
    }
    None
}
