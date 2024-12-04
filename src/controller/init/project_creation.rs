use std::{env, fs};
use std::path::Path;
use crate::controller::init::{build_toolkit, dev_toolkit, test_toolkit, ui_toolkit};
use crate::{NPM};
use cliclack::*;
use std::process::{exit, Command};

pub fn main(project_name: Option<String>) {
    let project_name = project_name.unwrap_or_else(get_project_name);
    
    let ui_toolkit_name = get_ui_toolkit_name;
    let dev_toolkit_name = get_dev_toolkit_name;
    let test_toolkit_name = get_test_toolkit_name;
    let build_toolkit_name = get_build_toolkit_name;
    let project_dir = make_svelte_project(project_name);

    add_ui_toolkit(ui_toolkit_name, &project_dir);
    add_dev_toolkit(dev_toolkit_name, &project_dir);
    add_test_toolkit(test_toolkit_name, &project_dir);
    add_build_toolkit(build_toolkit_name, &project_dir);
    open_project(project_dir);
}

fn get_project_name() -> String {
    intro("Project Creation").expect("Error while prompting intro");
    input("What is your project name?")
        .placeholder("./sparkling-solid")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a name.")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap()
}

fn get_ui_toolkit_name() -> &'static str {
    select("Pick a UI Toolkit".to_string())
        .initial_value("None")
        .item(
            "None",
            "None",
            "You don't want a( UI Toolkit because you are better",
        )
        .item(
            "tailwind",
            "Tailwind",
            "A utility-first CSS framework for rapid UI development.",
        )
        .item(
            "bootstrap",
            "Bootstrap",
            "A popular CSS framework with a lot of pre-built components.",
        )
        .item(
            "skeleton",
            "Skeleton",
            "A lightweight CSS framework for minimalistic designs.",
        )
        .item(
            "flowbite",
            "Flowbite",
            "A UI kit based on Tailwind CSS with ready-to-use components.",
        )
        .interact()
        .unwrap()
}

fn get_dev_toolkit_name() -> &'static str {
    select("Pick a Development Toolkit".to_string())
        .initial_value("None")
        .item(
            "None",
            "None",
            "You don't want a Developpment Toolkit because you are better",
        )
        .item(
            "eSLint",
            "ESLint",
            "A popular linter for identifying and fixing JavaScript/TypeScript issues.",
        )
        .item(
            "prettier",
            "Prettier",
            "An opinionated code formatter that supports multiple languages.",
        )
        .item(
            "husky",
            "Husky",
            "A tool to prevent bad `git commit` or `push` by running scripts before those actions.",
        )
        .item(
            "lint-staged",
            "Lint-staged",
            "Runs linters on pre-committed files in Git, useful with Husky for pre-commit hooks.",
        )
        .interact()
        .unwrap()
}

fn get_test_toolkit_name() -> &'static str {
    select("Pick a Testing Toolkit".to_string())
        .initial_value("None")
        .item(
            "None",
            "None",
            "You don't want a Testing Toolkit because you are better",
        )
        .item(
            "jest",
            "Jest",
            "A delightful JavaScript testing framework with a focus on simplicity.",
        )
        .item(
            "mocha",
            "Mocha",
            "A feature-rich JavaScript test framework running on Node.js and in the browser.",
        )
        .item(
            "cypress",
            "Cypress",
            "A JavaScript end-to-end testing framework for web applications.",
        )
        .item(
            "jasmine",
            "Jasmine",
            "A behavior-driven testing framework for JavaScript, often used with Angular.",
        )
        .interact()
        .unwrap()
}

fn get_build_toolkit_name() -> &'static str {
    select("Pick a Build & Deployment Toolkit".to_string())
        .initial_value("None")
        .item(
            "None",
            "None",
            "You don't want a Deployement Toolkit because you are better",
        )
        .item(
            "webpack",
            "Webpack",
            "A static module bundler for modern JavaScript applications.",
        )
        .item(
            "vite",
            "Vite",
            "A next-generation, fast build tool that focuses on speed and simplicity.",
        )
        .item(
            "parcel",
            "Parcel",
            "A zero-config, blazingly fast web application bundler.",
        )
        .item(
            "rollup",
            "Rollup",
            "A JavaScript module bundler optimized for ES modules.",
        )
        .interact()
        .unwrap()
}

fn add_ui_toolkit(toolkit_name: fn() -> &'static str, project_dir: &str) {
    match toolkit_name() {
        "skeleton" => ui_toolkit::skeleton::main(),
        "flowbite" => ui_toolkit::flowbite::main(),
        "tailwind" => ui_toolkit::tailwind::main(&project_dir),
        "bootstrap" => ui_toolkit::bootstrap::main(),
        _ => {}
    }
}

fn add_dev_toolkit(toolkit_name: fn() -> &'static str, _project_dir: &str) {
    match toolkit_name() {
        "eslint" => dev_toolkit::eslint::main(),
        "prettier" => dev_toolkit::prettier::main(),
        "husky" => dev_toolkit::husky::main(),
        "lint-staged" => dev_toolkit::lint_staged::main(),
        _ => {}
    }
}

fn add_test_toolkit(toolkit_name: fn() -> &'static str, _project_dir: &str) {
    match toolkit_name() {
        "jest" => test_toolkit::jest::main(),
        "mocha" => test_toolkit::mocha::main(),
        "cypress" => test_toolkit::cypress::main(),
        "jasmine" => test_toolkit::jasmine::main(),
        _ => {}
    }
}

fn add_build_toolkit(toolkit_name: fn() -> &'static str, _project_dir: &str) {
    match toolkit_name() {
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
        eprintln!("Error trying to clone SvelteKit template");
        exit(1);
    }

    let project_dir = format!("./{}", project_name);
    if !fs::metadata(&project_dir).is_ok() {
        eprintln!("The svelte project doesn't exist after creation");
        exit(1);
    }

    project_dir
}

fn open_project(project_name: String){
    let project_path = Path::new(&project_name);
    if !project_path.exists() {
        eprintln!("Directory {} doesn't exists", project_name);
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
