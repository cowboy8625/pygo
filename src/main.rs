mod arguments;
mod command;
mod defaults;
use anyhow::Result;
use command::PygoCommand;
use defaults::DEFAULT_GITIGNORE;
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

fn main() -> Result<()> {
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
            new_bin_project(&name)
        }
        PygoCommand::Init { name, ptype } => {
            if ptype {
                return init_lib_project(&name);
            }
            init_bin_project(&name)
        } // PygoCommand::Add(names) => {}
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Config {
    package: Package,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

fn run_command() -> Result<()> {
    let output = std::process::Command::new("python").arg("src").output()?;
    Ok(std::io::stdout().write_all(&output.stdout)?)
}

fn create_readme(name: &str) -> Result<()> {
    let mut file =
        File::create(format!("{}/README.md", name)).expect("Could not create __main__.py");
    let boilerplate = format!("# {}", name);
    file.write_all(boilerplate.as_bytes())?;
    Ok(())
}

fn init_git(name: &str) -> Result<()> {
    std::process::Command::new("git")
        .args(["init", name])
        .output()?;
    dot_git_ignore(name)?;
    Ok(())
}

fn dot_git_ignore(name: &str) -> Result<()> {
    std::fs::write(format!("{name}/.gitignore"), DEFAULT_GITIGNORE)?;
    Ok(())
}

fn create_main_project_dir(name: &str) {
    create_dir(name).expect("Could not create Poject Folder.");
}

fn create_src_dir(name: &str) -> Result<()> {
    create_dir(format!("{}/src", name))?;
    Ok(())
}

fn create_basic_setup(name: &str) -> Result<()> {
    create_main_project_dir(name);
    create_src_dir(name)?;
    create_readme(name)?;
    create_toml_file(name)?;
    Ok(())
}

fn create_src_file_with(project: &str, filename: &str, template: &str) -> Result<()> {
    let mut file = File::create(format!("{}/src/{}", project, filename))
        .unwrap_or_else(|_| panic!("Failed to create file {}.", filename));
    file.write_all(template.as_bytes())?;
    Ok(())
}

fn create_toml_file(name: &str) -> Result<()> {
    let mut file = File::create(format!("{}/Pygo.toml", name))
        .unwrap_or_else(|_| panic!("Failed to create file {}.", name));
    file.write_all(
        format!(
            "[project]\nname = \"{}\"
version = \"3.10.0\"
[dependencies]\n",
            name
        )
        .as_bytes(),
    )?;
    Ok(())
}

fn create_environment() -> Result<()> {
    // create_dir(&"env")?;
    Ok(())
}

fn new_bin_project(pro_dir: &str) -> Result<()> {
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

fn new_lib_project(pro_dir: &str) -> Result<()> {
    create_basic_setup(pro_dir)?;
    init_git(pro_dir)?;
    create_src_file_with(pro_dir, "__init__.py", "")?;
    create_environment()?;
    Ok(())
}

fn init_bin_project(pro_dir: &str) -> Result<()> {
    create_basic_setup(pro_dir)?;
    create_src_file_with(
        pro_dir,
        "__main__.py",
        "def main():\n    print(\"Hello World\")\n\nif __name__ == \"__main__\":\n    main()",
    )?;
    create_environment()?;
    Ok(())
}

fn init_lib_project(pro_dir: &str) -> Result<()> {
    create_basic_setup(pro_dir)?;
    create_src_file_with(pro_dir, "__init__.py", "")?;
    create_environment()?;
    Ok(())
}
