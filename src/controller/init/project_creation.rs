use std::{env, fs};
use std::path::Path;
use crate::controller::init::{build_toolkit, dev_toolkit, test_toolkit, ui_toolkit};
use crate::{NPM};
use cliclack::{input, select};
use colored::*;
use std::process::{exit, Command};

pub fn main(project_name: Option<String>) {
    let project_name = project_name.unwrap_or_else(get_project_name);

    println!("{}", "🔧 Setting up your project...".bold().yellow());

    let ui_toolkit_name = get_ui_toolkit_name();
    let dev_toolkit_name = get_dev_toolkit_name();
    let test_toolkit_name = get_test_toolkit_name();
    let build_toolkit_name = get_build_toolkit_name();
    let project_dir = make_svelte_project(project_name);

    add_ui_toolkit(ui_toolkit_name, &project_dir);
    add_dev_toolkit(dev_toolkit_name, &project_dir);
    add_test_toolkit(test_toolkit_name, &project_dir);
    add_build_toolkit(build_toolkit_name, &project_dir);

    println!("{}", "🚀 Project setup complete!".bold().green());

    open_project(project_dir);
}

fn get_project_name() -> String {
    println!("{}", "🌱 Project Creation".bold().cyan());

    let project_name = input("What is your project name?")
        .placeholder("./sparkling-solid".into())
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a name.".red().to_string())
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    project_name
}

fn get_ui_toolkit_name() -> &'static str {
    select("Pick a UI Toolkit".to_string())
        .initial_value("None")
        .item("None", "None", "No UI toolkit. It's all up to you!")
        .item("tailwind", "Tailwind", "A utility-first CSS framework for rapid UI development.")
        .item("bootstrap", "SvelteStrap", "A popular CSS framework for Svelte with a lot of pre-built components.")
        .item("skeleton", "Skeleton", "A lightweight CSS framework for minimalistic designs (Includes Tailwind).")
        .item("flowbite", "Flowbite", "A UI kit based on Tailwind CSS with ready-to-use components (Includes Tailwind).")
        .interact()
        .unwrap()
}

fn get_dev_toolkit_name() -> &'static str {
    select("Pick a Development Toolkit".to_string())
        .initial_value("None")
        .item("None", "None", "No development toolkit selected.")
        .item("eSLint", "ESLint", "A popular linter for JavaScript/TypeScript.")
        .item("prettier", "Prettier", "An opinionated code formatter for multiple languages.")
        .item("husky", "Husky", "Prevents bad `git commit` or `push` with pre-hooks.")
        .item("lint-staged", "Lint-staged", "Runs linters on pre-committed files in Git.")
        .interact()
        .unwrap()
}

fn get_test_toolkit_name() -> &'static str {
    select("Pick a Testing Toolkit".to_string())
        .initial_value("None")
        .item("None", "None", "No testing toolkit selected.")
        .item("jest", "Jest", "A JavaScript testing framework with simplicity in mind.")
        .item("mocha", "Mocha", "A feature-rich JavaScript test framework.")
        .item("cypress", "Cypress", "A modern testing framework for end-to-end tests.")
        .item("jasmine", "Jasmine", "A behavior-driven testing framework for JavaScript.")
        .interact()
        .unwrap()
}

fn get_build_toolkit_name() -> &'static str {
    select("Pick a Build & Deployment Toolkit".to_string())
        .initial_value("None")
        .item("None", "None", "No build toolkit selected.")
        .item("webpack", "Webpack", "A static module bundler for modern web apps.")
        .item("vite", "Vite", "A fast, next-generation build tool for modern web development.")
        .item("parcel", "Parcel", "A zero-config, blazing-fast web bundler.")
        .item("rollup", "Rollup", "A module bundler optimized for ES modules.")
        .interact()
        .unwrap()
}

fn add_ui_toolkit(toolkit_name: &str, project_dir: &str) {
    match toolkit_name.as_ref() {
        "skeleton" => ui_toolkit::skeleton::main(),
        "flowbite" => ui_toolkit::flowbite::main(),
        "tailwind" => ui_toolkit::tailwind::main(&project_dir),
        "bootstrap" => ui_toolkit::bootstrap::main(),
        _ => {}
    }
}

fn add_dev_toolkit(toolkit_name: &str, _project_dir: &str) {
    match toolkit_name.as_ref() {
        "eslint" => dev_toolkit::eslint::main(),
        "prettier" => dev_toolkit::prettier::main(),
        "husky" => dev_toolkit::husky::main(),
        "lint-staged" => dev_toolkit::lint_staged::main(),
        _ => {}
    }
}

fn add_test_toolkit(toolkit_name: &str, _project_dir: &str) {
    match toolkit_name.as_ref() {
        "jest" => test_toolkit::jest::main(),
        "mocha" => test_toolkit::mocha::main(),
        "cypress" => test_toolkit::cypress::main(),
        "jasmine" => test_toolkit::jasmine::main(),
        _ => {}
    }
}

fn add_build_toolkit(toolkit_name: &str, _project_dir: &str) {
    match toolkit_name.as_ref() {
        "webpack" => build_toolkit::webpack::main(),
        "vite" => build_toolkit::vite::main(),
        "parcel" => build_toolkit::parcel::main(),
        "rollup" => build_toolkit::rollup::main(),
        _ => {}
    }
}

fn make_svelte_project(project_name: String) -> String {
    let status = Command::new("git")
        .arg("clone")
        .arg("--quiet")
        .arg("https://github.com/gJmry/svelte-template")
        .arg(&project_name)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("{}", "Error trying to clone SvelteKit template".bold().red());
        exit(1);
    }

    let project_dir = format!("./{}", project_name);
    if !fs::metadata(&project_dir).is_ok() {
        eprintln!("{}", "The Svelte project doesn't exist after creation".bold().red());
        exit(1);
    }

    project_dir
}

fn open_project(project_name: String) {
    let project_path = Path::new(&project_name);
    if !project_path.exists() {
        eprintln!("{}", format!("Directory {} doesn't exist", project_name).bold().red());
        exit(1);
    }

    env::set_current_dir(project_path).expect("Failed to change directory");

    fs::remove_dir_all(".git").expect("Error when deleting .git directory");

    Command::new(NPM)
        .arg("run")
        .arg("dev")
        .status()
        .expect("Failed to execute command");
}
