//! Source file for the binary.
#[macro_use] extern crate clap;
#[macro_use] extern crate text_io;

extern crate time;
extern crate toml;
extern crate rustache;
extern crate project_init;
extern crate colored;



use colored::*;
use rustache::*;
use std::fs;
use clap::App;
use project_init::types::*;
use project_init::render::*;
use project_init::*;
use std::path::Path;
use time::strftime;

fn main() {

    // command-line parser
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    let force: bool = matches.occurrences_of("force") == 1 ;

    // set path to .pi.toml
    let mut path = std::env::home_dir()
        .expect("Couldn't determine home directory.");
    path.push(".pi.toml");

    // read global config file
    let decoded: Config = read_toml_config(path);
    
    // create author struct
    let author = 
        if let Some(aut) = decoded.author {
            aut 
        }
        else {
            let nam: String = read!("Enter your name: {}!");
            let ema: String = read!("Enter your email: {}!");
            Author { name: nam, email: ema, github_username: None }
        };
        
    //get year
    let now = time::now();
    let year = now.tm_year + 1900;
    let current_date = strftime("%m-%d-%Y", &now).unwrap();

    if let Some(matches_init) = matches.subcommand_matches("new") {

        // get project name
        let name = matches_init
            .value_of("name")
            .expect("Failed to supply project name");

        // get project template type
        let template_str = matches_init
            .value_of("template")
            .expect("Failed to supply project directory");

        // read template.toml
        let toml_file = match template_str {
            "rust" => includes::RUST_TEMPLATE,
            "vim" => includes::VIM_TEMPLATE,
            "python" => includes::PY_TEMPLATE,
            "haskell" => includes::HASK_TEMPLATE,
            _ => { println!("The requested template is not a built-in :(") ; std::process::exit(0x0f00) },
        };
        let parsed_toml = read_toml_str(toml_file.to_string(), "BUILTIN");
        let parsed_dirs = parsed_toml.files;
        let parsed_config = parsed_toml.config;
        
        // set license if it's set
        let (license_contents, license_name) =
            if let Some(l) = parsed_toml.license {
                match l.as_str() {
                    "BSD3" => (Some(includes::BSD3), "BSD3"),
                    "BSD" => (Some(includes::BSD), "BSD"),
                    "MIT" => (Some(includes::MIT), "MIT"),
                    "GPL3" => (Some(includes::GPL3), "GLP3"),
                    "AllRightsReserved" => (Some(includes::BSD3), "AllRightsReserved"),
                    _ => { println!("{}: requested license not found. Defaulting to AllRightsReserved","Warning".yellow()) 
                           ; (Some(includes::ALL_RIGHTS_RESERVED), "AllRightsReserved") }
                }
            }
            else if let Some(l) = decoded.license {
                match l.as_str() {
                    "BSD3" => (Some(includes::BSD3), "BSD3"),
                    "BSD" => (Some(includes::BSD), "BSD"),
                    "MIT" => (Some(includes::MIT), "MIT"),
                    "GPL3" => (Some(includes::GPL3), "GLP3"),
                    "AllRightsReserved" => (Some(includes::BSD3), "AllRightsReserved"),
                    _ => { println!("{}: requested license not found. Defaulting to AllRightsReserved","Warning".yellow()) 
                           ; (Some(includes::ALL_RIGHTS_RESERVED), "AllRightsReserved") }
                }
            }
            else {
                (None,"")
            };

        // set version
        // TODO only insert into the hash if necessary
        let version = 
            if let Some(config) = parsed_config.clone() {
                if let Some(v) = config.version {
                    v
                }
                else {
                    "0.1.0".to_string()
                }
            }
            else {
                println!("{}: no version info found, defaulting to '0.1.0'","Warning".yellow());
                "0.1.0".to_string()
            };

        // set github username to null if it's not provided
        let github_username = 
            if let Some(uname) = author.github_username {
                uname
            }
            else {
                println!("{}: No github username found, defaulting to null.", "Warning".yellow());
                "".to_string()
            };

        // Make a hash for inserting stuff into templates.
        let hash = HashBuilder::new().insert("project",name)
            .insert("year", year)
            .insert("name", author.name)
            .insert("version", version)
            .insert("email", author.email)
            .insert("github_username", github_username)
            .insert("license", license_name)
            .insert("date", current_date);
     
        // check if the directory exists and exit, if we haven't forced an overwrite.
        if Path::new(name).exists() && force == false {
            println!("Path '{}' already exists. Rerun with -f or --force to overwrite.", name);
            std::process::exit(0x0f00);
        };

        // create directories
        let _ = fs::create_dir(name);
        if let Some(dirs_pre) = parsed_dirs.directories {
            render_dirs(dirs_pre, &hash, name);
        }

        // Create files.
        let files =
            if let Some(files_pre) = parsed_dirs.files {
                render_files(files_pre, &hash, name)
            }
            else {
                VecBuilder::new()
            };

        // create license if it was asked for
        if let Some(lic) = license_contents {
            render_file(lic, name, "LICENSE", &hash);
        }

        // render readme if requested
        if let Some(readme) = parsed_toml.with_readme {
            if readme == true {
                render_file(includes::README, name, "README.md", &hash);
            }
        }
      
        let hash_with_files = HashBuilder::new()
            .insert("files", files);

        // render appropriate stuff by name.
        let _ = match template_str {
            "rust" => { write_file_plain(includes::RUST_LIB, name, "src/lib.rs");
                        write_file_plain(includes::RUST_TRAVIS_CI, name, ".travis.tml");
                        render_file(includes::CARGO_TOML, name, "Cargo.toml", &hash) },
            "vim" => render_file(includes::VIMBALL, name, "vimball.txt", &hash_with_files),
            "python" => { render_file(includes::PY_SETUP, name, "setup.py", &hash);
                          write_file_plain(includes::PY_CFG, name, "setup.cfg");
                          let mut bin_path = "bin/".to_string();
                          bin_path.push_str(name);
                          render_file(includes::PY_BIN, name, &bin_path, &hash); }
            "haskell" => { write_file_plain(includes::SETUP_HS, name, "Setup.hs");
                           write_file_plain(includes::MAIN, name, "app/Main.hs");
                           write_file_plain(includes::LIB, name, "src/Lib.hs");
                           write_file_plain(includes::BENCH, name, "bench/Bench.hs");
                           write_file_plain(includes::TEST, name, "test/Spec.hs");
                           render_file(includes::DEFAULT_NIX, name, "default.nix", &hash);
                           render_file(includes::RELEASE_NIX, name, "release.nix", &hash);
                           let mut cabal_path = name.to_string();
                           cabal_path.push_str(".cabal");
                           render_file(includes::CABAL, name, &cabal_path, &hash);
                           write_file_plain(includes::RELEASE_NIX, name, "release.nix");
                           write_file_plain(includes::STACK_YAML, name, "stack.yaml");
                           write_file_plain(includes::HASKELL_TRAVIS_CI, name, ".travis.yml"); }
            _ => std::process::exit(0x0f00),
        };

        // Print that we're done
        println!("Finished initializing project in {}/",name);

    }
    else if let Some(matches_init) = matches.subcommand_matches("init") {

        // get project name
        let name = matches_init
            .value_of("name")
            .expect("Failed to supply project name");

        // get project directory
        let project = matches_init
            .value_of("directory")
            .expect("Failed to supply project directory");

        // read template.toml for template
        let mut template_path = project.to_string();
        template_path.push_str("/template.toml");
        let parsed_toml = read_toml_dir(&template_path);
        let parsed_dirs = parsed_toml.files;
        let parsed_config = parsed_toml.config;
        
        // set license if it's set
        let (license_contents, license_name) =
            if let Some(l) = parsed_toml.license {
                match l.as_str() {
                    "BSD3" => (Some(includes::BSD3), "BSD3"),
                    "BSD" => (Some(includes::BSD), "BSD"),
                    "MIT" => (Some(includes::MIT), "MIT"),
                    "GPL3" => (Some(includes::GPL3), "GLP3"),
                    "AllRightsReserved" => (Some(includes::BSD3), "AllRightsReserved"),
                    _ => { println!("{}: requested license not found. Defaulting to AllRightsReserved","Warning".yellow()) 
                           ; (Some(includes::ALL_RIGHTS_RESERVED), "AllRightsReserved") }
                }
            }
            else if let Some(l) = decoded.license {
                match l.as_str() {
                    "BSD3" => (Some(includes::BSD3), "BSD3"),
                    "BSD" => (Some(includes::BSD), "BSD"),
                    "MIT" => (Some(includes::MIT), "MIT"),
                    "GPL3" => (Some(includes::GPL3), "GLP3"),
                    "AllRightsReserved" => (Some(includes::BSD3), "AllRightsReserved"),
                    _ => { println!("{}: requested license not found. Defaulting to AllRightsReserved","Warning".yellow()) 
                           ; (Some(includes::ALL_RIGHTS_RESERVED), "AllRightsReserved") }
                }
            }
            else {
                (None,"")
            };

        // set version
        // TODO only insert into the hash if necessary
        let version = 
            if let Some(config) = parsed_config.clone() {
                if let Some(v) = config.version {
                    v
                }
                else {
                    "0.1.0".to_string()
                }
            }
            else {
                println!("{}: no version info found, defaulting to '0.1.0'", "Warning".yellow());
                "0.1.0".to_string()
            };

        // set github username to null if it's not provided
        let github_username = 
            if let Some(uname) = author.github_username {
                uname
            }
            else {
                println!("{}: no github username found, defaulting to null", "Warning".yellow());
                "".to_string()
            };

        // Make a hash for inserting stuff into templates.
        let hash = HashBuilder::new().insert("project",name)
            .insert("year", year)
            .insert("name", author.name)
            .insert("version", version)
            .insert("email", author.email)
            .insert("github_username", github_username)
            .insert("license", license_name)
            .insert("date", current_date);
     
        // check if the directory exists and exit, if we haven't forced an overwrite.
        if Path::new(name).exists() && force == false {
            println!("Path '{}' already exists. Rerun with -f or --force to overwrite.", name);
            std::process::exit(0x0f00);
        };

        // create directories
        let _ = fs::create_dir(name);
        if let Some(dirs_pre) = parsed_dirs.directories {
            render_dirs(dirs_pre, &hash, name);
        }

        // create a list of files contained in the project, and create those files.
        // TODO should include templates/scripts/etc.
        let files =
            if let Some(files_pre) = parsed_dirs.files {
                render_files(files_pre, &hash, name)
            }
            else {
                VecBuilder::new()
            };

        // create license if it was asked for
        if let Some(lic) = license_contents {
            render_file(lic, name, "LICENSE", &hash);
        }

        // render readme if requested
        if let Some(readme) = parsed_toml.with_readme {
            if readme == true {
                render_file(includes::README, name, "README.md", &hash);
            }
        }

        // Make a hash for inserting stuff into templates.
        let hash_with_files = &hash
            .insert("files", files);

        // render templates
        render_templates(project, name, &hash_with_files, parsed_dirs.templates, false);

        // render scripts, i.e. files that should be executable.
        render_templates(project, name, &hash_with_files, parsed_dirs.scripts, true);

        // initialize version control
        if let Some(config) = parsed_config {
            if let Some(vc) = config.version_control {
                if vc == "git" {
                    repo::git_init(name);
                }
                else if vc == "hc" || vc == "mercurial" {
                    repo::hg_init(name);
                }
            }
        }
        else if let Some(vc) = decoded.version_control {
            if vc == "git" {
                repo::git_init(name);
            }
            else if vc == "hc" || vc == "mercurial" {
                repo::hg_init(name);
            }
        }

        // Print that we're done
        println!("Finished initializing project in {}/",name);

    }
}
