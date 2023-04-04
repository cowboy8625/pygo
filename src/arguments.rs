use crate::command::PygoCommand;
use crate::AUTHOR;
use anyhow::{anyhow, Result};

use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand};
pub fn cargs() -> Result<PygoCommand> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(AUTHOR)
        .about(crate_description!())
        .subcommand(SubCommand::with_name("run").about("Run your Python Project"))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add Python packages.")
                .arg(
                    Arg::with_name("lib-name")
                        .help("name of lib/package you want to down load.")
                        .takes_value(true)
                        .index(1),
                ),
        )
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
        .subcommand(
            SubCommand::with_name("init")
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

    if let Some(matches) = matches.subcommand_matches("add") {
        let _lib_name = matches.value_of("lib-name").unwrap_or("").to_string();
        // TODO: make this work.
        return Err(anyhow!("Command Not yet ready."));
    }
    if let Some(matches) = matches.subcommand_matches("new") {
        let name = matches
            .value_of("project_name")
            .unwrap_or("default")
            .to_string();
        let ptype = matches.is_present("library");
        return Ok(PygoCommand::New { name, ptype });
    }
    if let Some(matches) = matches.subcommand_matches("init") {
        let name = matches
            .value_of("project_name")
            .unwrap_or("default")
            .to_string();
        let ptype = matches.is_present("library");
        return Ok(PygoCommand::Init { name, ptype });
    }
    if let Some(_) = matches.subcommand_matches("run") {
        return Ok(PygoCommand::Run);
    }
    Err(anyhow!("No args given to pygo to run."))
}
