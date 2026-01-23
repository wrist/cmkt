use std::collections::HashMap;
use std::io::Result;
use std::process::Command;

use clap::{Parser, Subcommand};
use git2::Repository;
use serde::Serialize;
use std::{fs, path::Path};
use tera::{Context, Tera};
use toml::Value;
use toml_edit::{Array, DocumentMut, Item, value};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(help = "Run script defined in project.toml(alias of cmkt scripts)")]
    scripts: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new CMake project
    New {
        name: String,

        /// C++ version (default: 17)
        #[arg(long, default_value = "17")]
        cpp: String,

        /// Generator (default: Ninja)
        #[arg(long, default_value = "Ninja")]
        generator: String,

        /// executable|static|shared
        #[arg(long, default_value = "executable")]
        binary_type: String,

        /// Include test directory
        #[arg(long)]
        with_tests: bool,
    },
    /// Run script defined in project.toml
    Scripts { script: String },
    /// Add FetchContent
    Add {
        /// Repository name (user/repository or organization/repository)
        repo: String,

        /// Repository base url
        #[arg(short, long, default_value = "https://github.com")]
        base_url: String,

        /// Repository version(tag or branch)
        #[arg(short, long, default_value = "main")]
        tag: String,

        /// FetchContent declare or populate
        #[arg(short, long, default_value = "declare")]
        fetch_mode: String,

        /// Link library name
        #[arg(short, long, default_value=None)]
        lib_names: Option<Vec<String>>,
    },
}

#[derive(Serialize, Debug)]
struct PackageData {
    name: String,
    repo: String,
    base_url: String,
    tag: String,
    fetch_mode: String,
    lib_names: Vec<String>,
}

#[derive(Serialize)]
struct TemplateData {
    project_name: String,
    cpp_version: String,
    generator: String,
    binary_type: String,
    with_tests: bool,
}

fn run_command(cmd: &str) -> Result<()> {
    #[cfg(windows)]
    let status = Command::new("cmd").args(["/C", cmd]).status()?;

    #[cfg(not(windows))]
    let status = Command::new("sh").args(["-c", cmd]).status()?;

    if !status.success() {
        println!("command failed: {}", cmd);
    }
    Ok(())
}

fn dispatch_script(script: String) -> Result<()> {
    const TOML_FNAME: &str = "project.toml";

    let contents = fs::read_to_string(TOML_FNAME).expect("Failed to read project.toml");

    let config: HashMap<String, Value> =
        toml::from_str(&contents).expect("Could not convert to HashMap");

    //println!("{:?}", config);

    if let Some(table) = config.get("scripts") {
        if let Some(commands) = table.get(&script) {
            if let Some(commands) = commands.as_array() {
                for command in commands.iter() {
                    if let Some(command) = command.as_str() {
                        println!("Execute command: {:?}", command);
                        let _ = run_command(&command);
                    }
                }
            } else if let Some(command) = commands.as_str() {
                println!("Execute command: {:?}", command);
                let _ = run_command(&command);
            }
            //println!("{:?}", commands);
        } else {
            println!("script name {} doesn't exist in [scripts] section", &script);
        }
    } else {
        println!("[scripts] section doesn't exist in project.toml");
    }

    Ok(())
}

fn add_package(
    repo: String,
    base_url: String,
    tag: String,
    fetch_mode: String,
    mut lib_names: Option<Vec<String>>,
) -> Result<()> {
    let project_dir = Path::new(".");

    //let name = repo.replace("/", "_") + "_" + &tag.replace(".", "_");
    let name = repo.replace("/", "_");

    if lib_names.is_none() {
        let repo: Vec<_> = repo.split("/").collect();
        if repo.len() > 1 {
            lib_names = vec![repo[1].to_string()].into();
        } else {
            lib_names = vec![repo[0].to_string()].into();
        }
        //println!("lib names is none! => {:?}", lib_names);
    }

    let package = PackageData {
        name,
        repo,
        base_url,
        tag,
        fetch_mode,
        lib_names: lib_names.unwrap(),
    };

    //println!("{:?}", &package);
    //println!("{:?}", &package.lib_names);

    // manage project.toml
    const TOML_FNAME: &str = "project.toml";
    let contents = fs::read_to_string(TOML_FNAME).expect("Failed to read project.toml");
    let mut doc = contents.parse::<DocumentMut>().expect("invalid doc");

    let project_name = doc["package"]["name"].to_string();
    //println!("{}", &project_name);

    let deps = &mut doc["dependencies"];

    deps[&package.name]["repo"] = value(&package.repo);
    deps[&package.name]["base_url"] = value(&package.base_url);
    deps[&package.name]["tag"] = value(&package.tag);
    deps[&package.name]["fetch_mode"] = value(&package.fetch_mode);

    let lib_array = Array::from_iter(package.lib_names.iter());
    deps[&package.name]["lib_names"] = Item::Value(toml_edit::Value::Array(lib_array));
    //deps[&package.name]["lib_names"] = array();
    //for e in package.lib_names.iter() {
    //    println!("elem: {:?}", e);
    //    deps[&package.name]["lib_names"].as_array_mut().unwrap().push(e);
    //}

    //println!("Add dependency: {:?}", &package);
    println!("Current dependencies:\n{}", &deps);

    let mut packages: Vec<PackageData> = vec![];
    for (keys, value) in deps.as_table().unwrap().get_values() {
        //println!("Keys: {:?}", keys[0].get());
        //for(ks, vs) in value.as_inline_table().unwrap().get_values() {
        //    println!("Keys: {:?} Value: {:?}", ks[0].get(), vs.as_str().unwrap());
        //}

        //let kvs = value.as_inline_table().unwrap().get_values();
        //println!("Keys: {:?} Value: {:?}", kvs[0].0[0].get(), kvs[0].1.as_str().unwrap());
        //println!("Repo Value: {:?}", vs["repo"].as_str().unwrap());

        let vs = value.as_inline_table().unwrap();
        packages.push(PackageData {
            name: keys[0].get().to_string(),
            repo: vs["repo"].as_str().unwrap().to_string(),
            base_url: vs["base_url"].as_str().unwrap().to_string(),
            tag: vs["tag"].as_str().unwrap().to_string(),
            fetch_mode: vs["fetch_mode"].as_str().unwrap().to_string(),
            lib_names: vs["lib_names"]
                .as_array()
                .unwrap()
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        })
    }

    //println!("toml: {}", &doc);
    let _ = fs::write(TOML_FNAME, doc.to_string());

    // render
    let mut tera = Tera::default();

    let mut ctx = Context::new();
    ctx.insert("packages", &packages);
    ctx.insert("project_name", &project_name);

    let _ = tera.add_raw_template(
        "cmake/fetch.cmake.tera",
        include_str!("../templates/cmake/fetch.cmake.tera"),
    );
    let _ = tera.add_raw_template(
        "cmake/link.cmake.tera",
        include_str!("../templates/cmake/link.cmake.tera"),
    );

    render_file(
        &tera,
        "cmake/fetch.cmake.tera",
        project_dir.join("cmake/fetch.cmake"),
        &ctx,
    );
    render_file(
        &tera,
        "cmake/link.cmake.tera",
        project_dir.join("cmake/link.cmake"),
        &ctx,
    );

    Ok(())
}

fn create_project(
    name: String,
    cpp: String,
    generator: String,
    binary_type: String,
    with_tests: bool,
) {
    let project_dir = Path::new(&name);

    if project_dir.exists() {
        eprintln!("Error: directory '{}' already exists.", name);
        std::process::exit(1);
    }

    fs::create_dir(&project_dir).unwrap();
    fs::create_dir(project_dir.join("src")).unwrap();
    fs::create_dir(project_dir.join("build")).unwrap();
    fs::create_dir(project_dir.join("cmake")).unwrap();

    if with_tests {
        fs::create_dir(project_dir.join("tests")).unwrap();
    }

    let data = TemplateData {
        project_name: name.clone(),
        cpp_version: cpp,
        generator,
        binary_type: binary_type.clone(),
        with_tests,
    };

    let mut ctx = Context::new();
    ctx.insert("project_name", &data.project_name);
    ctx.insert("cpp_version", &data.cpp_version);
    ctx.insert("generator", &data.generator);
    ctx.insert("binary_type", &data.binary_type);
    ctx.insert("with_tests", &data.with_tests);

    let mut tera = Tera::default();

    let _ = tera.add_raw_template(
        "project.toml.tera",
        include_str!("../templates/project.toml.tera"),
    );
    //let _ = tera.add_raw_template("Makefile.tera", include_str!("../templates/Makefile.tera"));
    let _ = tera.add_raw_template(
        "CMakeLists.txt.tera",
        include_str!("../templates/CMakeLists.txt.tera"),
    );
    let _ = tera.add_raw_template(
        "CMakePresets.json.tera",
        include_str!("../templates/CMakePresets.json.tera"),
    );
    let _ = tera.add_raw_template(
        "README.md.tera",
        include_str!("../templates/README.md.tera"),
    );
    let _ = tera.add_raw_template(
        ".gitignore.tera",
        include_str!("../templates/.gitignore.tera"),
    );
    let _ = tera.add_raw_template(
        "src/CMakeLists.txt.tera",
        include_str!("../templates/src/CMakeLists.txt.tera"),
    );
    let _ = tera.add_raw_template(
        "src/main.cpp.tera",
        include_str!("../templates/src/main.cpp.tera"),
    );
    let _ = tera.add_raw_template(
        "src/lib.h.tera",
        include_str!("../templates/src/lib.h.tera"),
    );
    let _ = tera.add_raw_template(
        "src/lib.cpp.tera",
        include_str!("../templates/src/lib.cpp.tera"),
    );
    let _ = tera.add_raw_template(
        "tests/CMakeLists.txt.tera",
        include_str!("../templates/tests/CMakeLists.txt.tera"),
    );
    let _ = tera.add_raw_template(
        "tests/main.cpp.tera",
        include_str!("../templates/tests/main.cpp.tera"),
    );
    let _ = tera.add_raw_template(
        "cmake/fetch.cmake.tera",
        include_str!("../templates/cmake/fetch.cmake.tera"),
    );
    let _ = tera.add_raw_template(
        "cmake/link.cmake.tera",
        include_str!("../templates/cmake/link.cmake.tera"),
    );

    render_file(
        &tera,
        "project.toml.tera",
        project_dir.join("project.toml"),
        &ctx,
    );
    //render_file(&tera, "Makefile.tera",           project_dir.join("Makefile"),           &ctx);
    render_file(
        &tera,
        "CMakeLists.txt.tera",
        project_dir.join("CMakeLists.txt"),
        &ctx,
    );
    render_file(
        &tera,
        "CMakePresets.json.tera",
        project_dir.join("CMakePresets.json"),
        &ctx,
    );
    render_file(&tera, "README.md.tera", project_dir.join("README.md"), &ctx);
    render_file(
        &tera,
        ".gitignore.tera",
        project_dir.join(".gitignore"),
        &ctx,
    );
    render_file(
        &tera,
        "src/CMakeLists.txt.tera",
        project_dir.join("src/CMakeLists.txt"),
        &ctx,
    );

    render_file(
        &tera,
        "cmake/fetch.cmake.tera",
        project_dir.join("cmake/fetch.cmake"),
        &ctx,
    );
    render_file(
        &tera,
        "cmake/link.cmake.tera",
        project_dir.join("cmake/link.cmake"),
        &ctx,
    );

    if binary_type == "executable" {
        render_file(
            &tera,
            "src/main.cpp.tera",
            project_dir.join("src/main.cpp"),
            &ctx,
        );
    } else {
        render_file(&tera, "src/lib.h.tera", project_dir.join("src/lib.h"), &ctx);
        render_file(
            &tera,
            "src/lib.cpp.tera",
            project_dir.join("src/lib.cpp"),
            &ctx,
        );
    }

    if with_tests {
        render_file(
            &tera,
            "tests/CMakeLists.txt.tera",
            project_dir.join("tests/CMakeLists.txt"),
            &ctx,
        );
        render_file(
            &tera,
            "tests/main.cpp.tera",
            project_dir.join("tests/main.cpp"),
            &ctx,
        );
    }

    println!("Created project: {}", name);

    Repository::init(project_dir).expect("Failed to git init");
}

fn render_file(tera: &Tera, template: &str, out_path: impl AsRef<Path>, ctx: &Context) {
    let result = tera.render(template, ctx).unwrap();
    fs::write(out_path, result).unwrap();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New {
            name,
            cpp,
            generator,
            binary_type,
            with_tests,
        }) => create_project(name, cpp, generator, binary_type, with_tests),
        Some(Commands::Scripts { script }) => {
            dispatch_script(script).expect("Cannot dispatch script")
        }
        Some(Commands::Add {
            repo,
            base_url,
            tag,
            fetch_mode,
            lib_names,
        }) => add_package(repo, base_url, tag, fetch_mode, lib_names).expect("Cannot add package"),
        None => {
            if let Some(script) = cli.scripts {
                dispatch_script(script).expect("Cannot dispatch script")
            }
        }
    }
}
