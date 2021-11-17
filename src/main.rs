mod arguments;
mod command;
mod error;
use command::PygoCommand;
use error::PygoResult;
use std::fs::{create_dir, File};
use std::io::prelude::*;

use serde::Deserialize;

const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

fn main() -> PygoResult<()> {
    let _config: Option<Config> = std::fs::read_to_string("Pygo.toml")
        .ok()
        .and_then(|f| toml::from_str(&f).ok());
    let command = arguments::cargs()?;
    match command {
        PygoCommand::Run => run_command(),
        PygoCommand::New { name, ptype } => {
            if ptype {
                return new_lib_project(&name);
            }
            return new_bin_project(&name);
        }
        PygoCommand::Init { name, ptype } => {
            if ptype {
                return init_lib_project(&name);
            }
            return init_bin_project(&name);
        } // PygoCommand::Add(names) => {}
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

fn run_command() -> PygoResult<()> {
    let output = std::process::Command::new("python").arg("src").output()?;
    Ok(std::io::stdout().write_all(&output.stdout)?)
}

fn create_readme(name: &str) -> PygoResult<()> {
    let mut file =
        File::create(format!("{}/README.md", name)).expect("Could not create __main__.py");
    let boilerplate = format!("# {}", name);
    file.write_all(boilerplate.as_bytes())?;
    Ok(())
}

fn init_git(name: &str) -> PygoResult<()> {
    std::process::Command::new("git")
        .args(&["init", name])
        .output()?;
    dot_git_ignore(name)?;
    Ok(())
}

fn dot_git_ignore(name: &str) -> PygoResult<()> {
    let mut file = File::create(format!("{}/.gitignore", name))?;
    let boilerplate = format!("# {}", name);
    file.write_all(boilerplate.as_bytes())?;
    Ok(())
}

fn create_main_project_dir(name: &str) {
    create_dir(name).expect("Could not create Poject Folder.");
}

fn create_src_dir(name: &str) -> PygoResult<()> {
    create_dir(format!("{}/src", name))?;
    Ok(())
}

fn create_basic_setup(name: &str) -> PygoResult<()> {
    create_main_project_dir(name);
    create_src_dir(name)?;
    create_readme(name);
    create_toml_file(name)?;
    Ok(())
}

fn create_src_file_with(project: &str, filename: &str, template: &str) -> PygoResult<()> {
    let mut file = File::create(format!("{}/src/{}", project, filename))
        .expect(&format!("Failed to create file {}.", filename));
    file.write_all(template.as_bytes())?;
    Ok(())
}

fn create_toml_file(name: &str) -> PygoResult<()> {
    let mut file = File::create(&format!("{}/Pygo.toml", name))
        .expect(&format!("Failed to create file {}.", name));
    file.write_all(
        &format!(
            "[project]\nname = \"{}\"
version = \"3.10.0\"
[dependencies]\n",
            name
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn create_environment() -> PygoResult<()> {
    create_dir(&"env")?;
    Ok(())
}

fn new_bin_project(pro_dir: &str) -> PygoResult<()> {
    create_basic_setup(pro_dir)?;
    init_git(pro_dir)?;
    create_src_file_with(
        pro_dir,
        "__main__.py",
        "def main():\n    print(\"Hello World\")\n\nif __name__ == \"__main__\":\n    main()",
    )?;
    create_environment()?;
    Ok(())
}

fn new_lib_project(pro_dir: &str) -> PygoResult<()> {
    create_basic_setup(pro_dir)?;
    init_git(pro_dir)?;
    create_src_file_with(pro_dir, "__init__.py", "")?;
    create_environment()?;
    Ok(())
}

fn init_bin_project(pro_dir: &str) -> PygoResult<()> {
    create_basic_setup(pro_dir)?;
    create_src_file_with(
        pro_dir,
        "__main__.py",
        "def main():\n    print(\"Hello World\")\n\nif __name__ == \"__main__\":\n    main()",
    )?;
    create_environment()?;
    Ok(())
}

fn init_lib_project(pro_dir: &str) -> PygoResult<()> {
    create_basic_setup(pro_dir)?;
    create_src_file_with(pro_dir, "__init__.py", "")?;
    create_environment()?;
    Ok(())
}
